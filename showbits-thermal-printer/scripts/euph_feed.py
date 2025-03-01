import argparse
import json
import time

import requests
import websockets.sync.client as ws


class Room:
    def __init__(self, addr, name, nick):
        self.addr = addr
        self.name = name
        self.nick = nick

        self.send_reply_id = None
        self.print_thread_mid = None

        self.ws = None
        self.next_id = 0

    def run(self):
        while True:
            self.ws = ws.connect(f"wss://euphoria.leet.nu/room/{self.name}/ws")

            try:
                while True:
                    packet = self.ws.recv()
                    packet = json.loads(packet)
                    self._on_packet(packet)
            except Exception as e:
                print("Oop:", e)
                time.sleep(10)

        print("Stopped")

    def _send(self, ptype, **data):
        cur_ws = self.ws
        if cur_ws is None:
            return

        pid = f"{self.next_id}"
        self.next_id += 1

        packet = {
            "id": pid,
            "type": ptype,
            "data": data,
        }
        packet = json.dumps(packet)
        cur_ws.send(packet)

        return pid

    def _on_packet(self, packet):
        data = packet.get("data", {})

        match packet["type"]:
            case "ping-event":
                self._on_ping_event(data)
            case "snapshot-event":
                self._on_snapshot_event(data)
            case "send-reply":
                self._on_send_reply(packet)
            case "send-event":
                self._on_send_event(data)

    def _on_ping_event(self, data):
        self._send("ping-reply", time=data["time"])

    def _on_snapshot_event(self, data):
        self._send("nick", name=self.nick)
        self.send_reply_id = self._send(
            "send", content="Direct replies to this message will be printed."
        )

    def _on_send_reply(self, packet):
        if packet.get("id") == self.send_reply_id:
            print("Received send-reply")
            self.print_thread_mid = packet["data"]["id"]

    def _on_send_event(self, data):
        if not self.print_thread_mid:
            return

        if self.print_thread_mid != data.get("parent"):
            return

        data = {
            "username": data["sender"]["name"],
            "content": data["content"],
        }

        print("Posting", data)
        try:
            requests.post(f"http://{self.addr}/chat", data=data)
        except Exception as e:
            print("Oop:", e)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("addr")
    parser.add_argument("room")
    parser.add_argument("nick")
    args = parser.parse_args()

    room = Room(args.addr, args.room, args.nick)
    room.run()


if __name__ == "__main__":
    main()
