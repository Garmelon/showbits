import argparse
import random
import re
import time

import requests
import websockets.sync.client as ws


class IrcMessage:
    def __init__(self, tags, source, command, params):
        self.tags = tags
        self.source = source
        self.command = command
        self.params = params


def _parse_tags(raw):
    tags = {}

    for pair in raw.split(";"):
        parts = pair.split("=", maxsplit=1)
        if len(parts) == 1:
            [key] = parts
            tags[key] = ""
        else:  # len(parts) == 2
            [key, value] = parts
            tags[key] = value

    return tags


def _parse_params(raw):
    parts = raw.split(":", maxsplit=1)
    params = parts[0].split()

    if len(parts) == 2:
        params.append(parts[1])

    return params


LINE_RE = re.compile(
    r"(@(?P<tags>\S+)\s+)?"
    r"(:(?P<source>\S+)\s+)?"
    r"(?P<command>\S+)"
    r"(\s+(?P<params>.*))?"
)


def parse_line(line):
    m = LINE_RE.fullmatch(line)

    tags = {}
    if m_tags := m.group("tags"):
        tags = _parse_tags(m_tags)

    source = None
    if m_source := m.group("source"):
        source = m_source

    command = m.group("command")

    params = []
    if m_params := m.group("params"):
        params = _parse_params(m_params)

    return IrcMessage(tags, source, command, params)


def parse_lines(lines):
    msgs = []

    for line in lines.split("\r\n"):
        if line:
            msgs.append(parse_line(line))

    return msgs


class Room:
    def __init__(self, addr, channel):
        self.addr = addr
        self.channel = channel

        self.ws = None

    def run(self):
        while True:
            self.ws = ws.connect("wss://irc-ws.chat.twitch.tv/")

            try:
                self.join()
                while True:
                    packet = self.ws.recv()
                    self._on_packet(packet)
            except Exception as e:
                print("Oop:", e)
                time.sleep(10)

    def join(self):
        nick = f"justinfan{random.randint(1, 99999)}"
        print(f"Joining #{self.channel} as {nick}")
        self.ws.send("CAP REQ :twitch.tv/tags twitch.tv/commands")
        self.ws.send("PASS SCHMOOPIIE")  # What the web ui does
        self.ws.send(f"NICK {nick}")
        self.ws.send(f"USER {nick} 8 * :{nick}")  # Web ui sends 8, not 0
        self.ws.send(f"JOIN #{self.channel}")

    def _on_packet(self, packet):
        msgs = parse_lines(packet)
        for msg in msgs:
            if msg.command != "PRIVMSG":
                continue

            data = {
                "username": msg.tags["display-name"],
                "content": msg.params[1],
            }

            print("Posting", data)
            try:
                requests.post(f"http://{self.addr}/chat", data=data)
            except Exception as e:
                print("Oop:", e)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("addr")
    parser.add_argument("channel")
    args = parser.parse_args()

    room = Room(args.addr, args.channel)
    room.run()


if __name__ == "__main__":
    main()
