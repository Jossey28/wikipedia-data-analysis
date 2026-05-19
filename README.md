# Wikipedia Data Analytics

## Context

This project idea was spawned from watching adumb's graph of wikipedia video

[![Youtube Video Preview and Link](https://img.youtube.com/vi/JheGL6uSF-4/0.jpg)](https://www.youtube.com/watch?v=JheGL6uSF-4)

I enjoy watching videos about data analytics so I thought, `"why not try it myself" :)`

## Project Set-Up (Windows)
### Download and Extract Data
```pwsh
# Create the data and raw directories if they don't exist
New-Item -Path ".\data" -ItemType Directory -Force
New-Item -Path ".\raw" -ItemType Directory -Force

# Download seperate data sources
## Article Titles and IDs 
Invoke-WebRequest "https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-page.sql.gz" -OutFile ".\raw\enwiki-latest-page.sql.gz" 

## Internal Links between articles (e.g graph edges in the video)
Invoke-WebRequest "https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-pagelinks.sql.gz" -OutFile ".\raw\enwiki-latest-pagelinks.sql.gz"

## Redirect mappings, maps alises to article titles
Invoke-Webrequest "https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-redirect.sql.gz" -OutFile ".\raw\enwiki-latest-redirect.sql.gz"

# Install 7za
winget install 7zip.7zip

# Extract the data into .sql files
Get-ChildItem ".\raw\*.gz" | ForEach-Object { 
    & "7za.exe" x $_.FullName -o ".\data\" -y 
}
```
### Download database server for quick enumeration 
```pwsh
Invoke-WebRequest "https://dev.mysql.com/get/Downloads/MySQLInstaller/mysql-installer-community-8.0.46.0.msi" -OutFile "$env:TEMP\mysql-installer.msi"

& "$env:TEMP\mysql-installer.msi" # I just did full install


mysqlsh --sql -u root -p -e "CREATE DATABASE wikipedia;"

# These 3 commands can take a varying amount of time, with the size being ~40GB of data being imported
mysql -u root -p wikipedia --execute "SOURCE data/enwiki-latest-page.sql"
mysql -u root -p wikipedia --execute "SOURCE data/enwiki-latest-redirect.sql"
mysql -u root -p wikipedia --execute "SOURCE data/enwiki-latest-pagelinks.sql"
```