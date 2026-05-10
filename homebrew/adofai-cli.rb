class AdofaiCli < Formula
  desc "Cinematic terminal-based A Dance of Fire and Ice engine"
  homepage "https://3289david.github.io/adofai-cli"
  version "1.0.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/3289david/adofai-cli/releases/download/v1.0.0/adofai-cli-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM64"
    else
      url "https://github.com/3289david/adofai-cli/releases/download/v1.0.0/adofai-cli-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X64"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/3289david/adofai-cli/releases/download/v1.0.0/adofai-cli-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_ARM64"
    else
      url "https://github.com/3289david/adofai-cli/releases/download/v1.0.0/adofai-cli-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_X64"
    end
  end

  def install
    bin.install "adofai"
  end

  test do
    assert_match "adofai", shell_output("#{bin}/adofai --version")
  end
end
