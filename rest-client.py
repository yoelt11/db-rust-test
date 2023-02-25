import requests

if __name__=="__main__":
    URL = "http://127.0.0.1:8080/echo"
    POST = "Hello"

    r = requests.post(url=URL, data=POST, verify=False)
    print(r.text)
