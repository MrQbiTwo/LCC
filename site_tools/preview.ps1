param(
    [ValidateSet('build', 'serve')][string]$Mode = 'serve',
    [string]$Ruby = 'ruby',
    [int]$Port = 4173
)
$ErrorActionPreference = 'Stop'
$siteRoot = Split-Path -Parent $PSScriptRoot
$savedGemHome = $env:GEM_HOME
$savedGemPath = $env:GEM_PATH
$savedRubyOpt = $env:RUBYOPT
Push-Location $siteRoot
try {
    # Reuse the isolated local installation when present; ordinary Bundler also works.
    if (Test-Path -LiteralPath "$siteRoot/build/gems/specifications/jekyll-4.4.1.gemspec") {
        $defaultGemPath = & $Ruby -e 'puts Gem.path.join(File::PATH_SEPARATOR)'
        $env:GEM_HOME = "$siteRoot/build/gems"
        $env:GEM_PATH = "$siteRoot/build/gems;$defaultGemPath"
    }
    $env:RUBYOPT = '-W0'
    $jekyllArgs = @($Mode)
    if ($Mode -eq 'serve') {
        $jekyllArgs += @('--host', '127.0.0.1', '--port', "$Port", '--no-watch')
    }
    & $Ruby -e "load Gem.bin_path('jekyll', 'jekyll')" -- @jekyllArgs
    if ($LASTEXITCODE -ne 0) { throw "Jekyll failed with exit code $LASTEXITCODE" }
} finally {
    Pop-Location
    $env:GEM_HOME = $savedGemHome
    $env:GEM_PATH = $savedGemPath
    $env:RUBYOPT = $savedRubyOpt
}
