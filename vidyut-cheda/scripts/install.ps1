Set-Location ..

if (Test-Path "data") {
	Remove-Item -LiteralPath data -Force -Recurse
}

if ((Test-Path "data-git") -eq $false) {
	git clone --depth=1 https://github.com/sanskrit/data.git data-git
}

python3 data-git\bin\make_data.py --make_prefixed_verbals

if (Test-Path "data-git") {
	Copy-Item -Path "data-git/all-data" -Destination "data" -Force -Recurse 
	Remove-Item -LiteralPath "data-git" -Force -Recurse
}

New-Item -Path "data/vidyut-0.1.0" -ItemType Directory

make train 
make create_kosha
make generate_sandhi_rules

cargo run --release --bin cheda -- "saMskftam" --data-dir "data/vidyut-0.1.0"

Write-Output "                       "
Write-Output "-----------------------"
Write-Output "Vidyut is ready for use"
Write-Output "------------------------"
