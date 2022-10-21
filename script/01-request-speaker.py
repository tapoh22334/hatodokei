import requests


def request_speaker():
    host = 'localhost'
    port = 50031
    return requests.get(
        f'http://{host}:{port}/speakers'
    )


if __name__ == "__main__":
    response = request_speaker()
    print(response.text)
