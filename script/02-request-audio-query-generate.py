import sys
import json
import requests


def request_query_generate(text: str, speaker: str):
    host = 'localhost'
    port = 50031

    params = {
            'text': text,
            'speaker': speaker,
        }

    audio_query = requests.post(
        f'http://{host}:{port}/audio_query',
        params=params
    )

    print(json.dumps(audio_query.json()))


if __name__ == "__main__":
    # sentence = sys.stdin.readline().rstrip('\n')
    speaker = sys.argv[1]
    sentence = sys.argv[2]
    request_query_generate(sentence, speaker)
