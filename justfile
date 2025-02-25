# just command runner configuration for the pavex-diary repo

# if on Windows, set the shell accordingly
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# run cargo px check
check:
  cargo px check

# run cargo fmt
fmt:
  cargo fmt