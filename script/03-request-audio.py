import sys
import requests


def request_audio(text: str, speaker: str, audio_query: str):
    host = 'localhost'
    port = 50031

    params = {
            'text': text,
            'speaker': speaker,
        }

    response_wav = requests.post(
        f'http://{host}:{port}/synthesis',
        headers={'Content-Type': 'application/json', },
        params=params,
        data=audio_query
    )

    return response_wav


if __name__ == "__main__":
    # sentence = sys.stdin.readline().rstrip('\n')
    speaker = sys.argv[1]
    sentence = sys.argv[2]

    with open(sys.argv[3]) as f:
        audio_query = f.read()

    print(audio_query)
    response = request_audio(sentence, speaker, audio_query)

    print(response)

    with open(f"{sentence}.wav", "wb") as file:
        file.write(response.content)
