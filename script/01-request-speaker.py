import requests
import json


def request_speaker(port):
    host = 'localhost'
    return requests.get(
        f'http://{host}:{port}/speakers'
    )


def print_speaker(f, port, speaker):
    styles = speaker["styles"]
    for style in styles:
        f.write("%d:%s:%s:%s\n" %
                (port, speaker["name"], style["name"], style["id"]))


if __name__ == "__main__":
    ports = [50031, 50021]

    with open("speakers.txt", "w", encoding="utf-8") as f:

        for port in ports:
            response = request_speaker(port)
            speakers = json.loads(response.text)

            for speaker in speakers:
                print_speaker(f, port, speaker)
