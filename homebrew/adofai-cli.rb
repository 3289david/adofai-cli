class AdofaiCli < Formula
  desc "Cinematic terminal-based A Dance of Fire and Ice engine"
  homepage "https://3289david.github.io/adofai-cli"
  version "1.0.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/3289david/adofai-cli/releases/download/v1.0.1/adofai-cli-aarch64-apple-darwin.tar.gz"
      sha256 "a341e3c47ae73be286975b810a98a1b7ef8ee8494f6666d98c691e15cd9fbefe"
    else
      url "https://github.com/3289david/adofai-cli/releases/download/v1.0.1/adofai-cli-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X64"
    end
  end

  on_linux do
    url "https://github.com/3289david/adofai-cli/releases/download/v1.0.1/adofai-cli-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "cfd113bf38a784f313e13cbd050ba3b6dcd73e398789648928c7fd9d28d0187e"
  end

  def install
    bin.install "adofai"
  end

  test do
    assert_match "adofai", shell_output("#{bin}/adofai --version")
  end
end
