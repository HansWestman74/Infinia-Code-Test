# Infinia code test

To run:
- Start server: 
    - cd into server folder
    - Start actix server ```cargo r -r```
- Run client
    - cd into client folder
    - Run client ```cargo r -r```

To autogenerate documentation:
- For server crate: 
    - cd into server folder
    - Generate documentation ```cargo doc --no-deps```
    - Open ```server/target/doc/actix_webtask_service/all.html``` in any browser
- For client crate
    - cd into client folder
    - Generate documentation ```cargo doc --no-deps```
    - open ```client/target/doc/client/all.html``` in any browser