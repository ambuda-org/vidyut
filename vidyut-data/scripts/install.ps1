if (Test-Path "data-git") {
	Remove-Item -LiteralPath "data-git" -Force -Recurse
}

if (Test-Path "dcs-data") {
	Remove-Item -LiteralPath "dcs-data" -Force -Recurse
}

Set-Location ..

Write-Output "------------------------"
Write-Output "|    DCS Corpus Data   |"
Write-Output "------------------------"

if (Test-Path ".\data\raw\dcs") {
	Write-Output "Training data already exists -- skipping fetch"
}
else {
	Write-Output "Training data does not exist -- fetching"
	New-Item -Path "data\raw\dcs" -ItemType Directory
	git clone --depth 1 https://github.com/OliverHellwig/sanskrit.git dcs-data
	Copy-Item -Path ".\dcs-data\dcs\data\conllu" -Destination ".\data\raw\dcs\conllu" -Force -Recurse
	Remove-Item -LiteralPath "dcs-data" -Force -Recurse
}

Write-Output "-----------------------------"
Write-Output "|   Linguistic Data Fetch   |"
Write-Output "-----------------------------"

if (Test-Path ".\data\raw\lex") {
	Write-Output "Lexical data already exists -- skipping fetch"
}
else {
	Write-Output "Lexical data does not exist -- fetching"
	New-Item -Path ".\data\raw\lex" -ItemType Directory
	git clone --depth=1 https://github.com/sanskrit/data.git data-git
    python data-git/bin/make_data.py --make_prefixed_verbals
	Copy-Item -Path ".\data-git\all-data\*" -Destination "data\raw\lex"
	Remove-Item -LiteralPath "data-git" -Force -Recurse
}

Write-Output "-----------------------------"
Write-Output "|        Vidyut Build       |"
Write-Output "-----------------------------"

make create_kosha
make test_kosha
make create_sandhi_rules
make train_cheda
make eval_cheda

Write-Output "                       "
Write-Output "-------------------------"
Write-Output " Vidyut is ready for use "
Write-Output "-------------------------"
