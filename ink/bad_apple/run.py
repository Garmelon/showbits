import cv2

WINDOW = "webcam"

cv2.namedWindow(WINDOW)
cam = cv2.VideoCapture(0)

while True:
    ret, frame = cam.read()
    cv2.imshow(WINDOW, frame)
    if cv2.waitKey(20) == 27:  # Esc
        break

cam.release()
cv2.destroyWindow(WINDOW)
