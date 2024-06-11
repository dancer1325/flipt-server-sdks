Flipt server with a booleanFlag + Spring Boot app

## How to run locally?
* [Install & run Flipt server](https://docs.flipt.io/self-hosted/overview)
  * via Docker 
    ```
    docker run -d 
      -p 8080:8080 -p 9000:9000
      -v $HOME/flipt:/var/opt/flipt 
      docker.flipt.io/flipt/flipt:latest
    ```
* `mvn spring-boot:run`

## How to test it?
* Create a boolean flag, named 'flag_boolean' via the Flipt UI
  * [Follow this guide](https://docs.flipt.io/introduction#create-a-flag)
* Open in your desired browser 'localhost:9000' and
  * check that you always get '[{"id":0}]'
  * open again FliptUI and disableit
  * hit again in the browser, always getting '[{"id":2}]'

## Notes
* 'Book.getId()'
  * required to create the bean