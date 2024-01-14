from lib.hello import Greeting

# greeting = Greeting(
# 	field1 = "field1",
# 	field2 = "field2",
# 	field3 = "field3",
# )

# with open("test.bin", "wb") as file:
# 	nbytes = file.write(bytes(greeting))
# 	print(nbytes)

# with open("test.json", "w") as file:
# 	file.write(greeting.to_json())
	

with open("test.bin", "rb") as file:
	bytes = file.read()
	print(bytes)

	greeting = Greeting().parse(bytes)
	print(greeting)
	
with open("test.json", "r") as file:
	s = file.read()
	greeting = Greeting().from_json(s)
	print(greeting)
	

