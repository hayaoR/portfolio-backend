Param ($SKIP_DOCKER)
$ErrorActionPreference = "STOP"
if ( !(Get-Command psql -ea SilentlyContinue) ) {
    Write-Host "Error: psql is not installed"
    exit
}

if ( !(Get-Command sqlx -ea SilentlyContinue) ) {
    Write-Host "Error: sqlx is not installed"
    exit
}

$DB_USER="postgres"
$DB_PASSWORD="password"
$DB_NAME="portfolio"
$DB_PORT="5432"

if (-not $SKIP_DOCKER) {
    docker run -e POSTGRES_USER=$DB_USER -e POSTGRES_PASSWORD=$DB_PASSWORD -e POSTGRES_DB=$DB_NAME -p "${DB_PORT}:5432" -d postgres postgres -N 1000
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Docker throws error"
        exit
    } 
}

$ENV:PGPASSWORD="${DB_PASSWORD}"
$i = 0
$max = 100
while ($i -le $max) {
        psql.exe -h "localhost" -U $DB_USER -p $DB_PORT -d "postgres" -c '\q'
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Postgres is unavailable sleeping" 
            Start-Sleep 1
        } else {
            break
        }
    $i += 1
}

$Env:DATABASE_URL = "postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
sqlx database create
if ($LASTEXITCODE -ne 0) {
    Write-Host "sqlx database create throws error"
    exit
} 
sqlx migrate run
if ($LASTEXITCODE -ne 0) {
    Write-Host "sqlx migrate run throws error"
    exit
} 

Write-Host "Postgres has been migrated, ready to go!"