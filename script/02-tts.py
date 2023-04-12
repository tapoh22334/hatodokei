import json
import requests

sentencies = [
    "0時です",
    "1時です",
    "2時です",
    "3時です",
    "4時です",
    "5時です",
    "6時です",
    "7時です",
    "8時です",
    "9時です",
    "10時です",
    "11時です",
    "12時です",
    "13時です",
    "14時です",
    "15時です",
    "16時です",
    "17時です",
    "18時です",
    "19時です",
    "20時です",
    "21時です",
    "22時です",
    "23時です"
]


def request_audio(port: int, speaker: str, audio_query: str):
    host = 'localhost'

    params = {
            'speaker': speaker,
        }

    response_wav = requests.post(
        f'http://{host}:{port}/synthesis',
        headers={'Content-Type': 'application/json', },
        params=params,
        data=audio_query
    )

    return response_wav


def request_query_generate(port: str, text: str, speaker: str):
    host = 'localhost'

    params = {
            'text': text,
            'speaker': speaker,
        }

    audio_query = requests.post(
        f'http://{host}:{port}/audio_query',
        params=params
    )

    res = json.dumps(audio_query.json())

    return res


if __name__ == "__main__":
    speakers = []
    with open('speakers.txt', 'r', encoding="utf-8", newline='\n') as f:
        for line in f:
            line = line.strip().split(':')
            d = (line[0], line[1], line[2], line[3])
            speakers.append(d)

    for sentence in sentencies:
        for port, name, style, id in speakers:
            print(port, name, style, id)
            audio_query = request_query_generate(port, sentence, id)
            print(audio_query)
            response = request_audio(port, id, audio_query)
            print(response)
            with open(f"wav/{port}-{id}-{sentence}.wav", "wb") as f:
                f.write(response.content)
