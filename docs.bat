Xcopy target\\doc docs\\ /S /Q /Y
echo|set /p="<!DOCTYPE html><html><head></head><body><script>window.location.href = "../docs/toylang/index.html"</script></body></html>" > docs/index.html