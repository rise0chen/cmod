import hello

print(hello.hello_world())

xiaoming = hello.hello_human("xiaoming")
print(xiaoming.hello())
xiaohong = hello.Human.new("xiaohong")
print(xiaohong.hello())
anon = hello.Human.anon()
print(anon.hello())

print(hello.say.bye())
