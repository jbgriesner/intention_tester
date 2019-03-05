# Intention tester

Test intention/category detection api against test files (in the `./data` folder).

You can test your API in 2 steps with Docker:

- build the intention_tester image:
```
docker build -t intention_tester:1.0 .
```

- launch the tests:
```
docker run intention_tester:1.0 <api-url>
```
