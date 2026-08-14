class Envdoctor < Formula
  desc "Automated Developer Environment Diagnostic & Health Check Tool"
  homepage "https://github.com/sha256san/envdr"
  url "https://github.com/sha256san/envdr.git", tag: "v0.3.2"
  license "MIT"
  head "https://github.com/sha256san/envdr.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/envdoctor"
    bin.install "target/release/envdr"
  end

  test do
    system "#{bin}/envdoctor", "--version"
    system "#{bin}/envdr", "--version"
  end
end
