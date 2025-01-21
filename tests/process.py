import threading

def client1():
    print("client 1 here")

def client2():
    print("client 2 here")

ps = [threading.Thread(target=f) for f in [client1, client2]]
for p in ps:
    p.start()
for p in ps:
    p.join()
