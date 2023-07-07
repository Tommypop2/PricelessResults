# Priceless Results

## About

The backend server for the Priceless Results system. This is currently WIP, so many of the API routes aren't entirely guarded

## How to build and run

- First ensure that cargo and rust are installed from the rust website: <https://www.rust-lang.org/learn/get-started>
- Install surrealdb <https://surrealdb.com/docs/installation/windows>. Chocolatey or scoop can also be used, whilst ensuring that the version installed matches the version within `cargo.toml`
- Install openssl, either from <https://wiki.openssl.org/index.php/Binaries>, or by using a package manager such as chocolatey or winget
- Run the database by running `./start_db.bat`, which just runs `surreal start --log debug --user root --pass root memory`. This creates a database in RAM, as a persistent database isn't required for testing
- Run the actix web server with `./run.bat` - ensuring that the `OPENSSL_DIR` environment variable set within it points to the correct path.
- The server should now be running at <http://localhost:8080/>, and you can test if it's working by going to this address in your browser.
  You can edit or see either batch file by right clicking and selecting `edit` on windows
