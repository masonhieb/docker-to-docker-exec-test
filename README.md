# docker-to-docker-exec-test
Testing the concept of having a program on docker container A execute a program on docker container B and return the results

# Server

- REST interface, runs the executor on the executor's container and returns the result

# Executor

- Its entire purpose is to run a simple program and return a result, to be gathered by the server. In this case, the program takes a string item and returns its accompanying item (ie meat->potato, strawberries->cream, etc)
