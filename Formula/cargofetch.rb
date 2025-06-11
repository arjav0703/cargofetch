class Cargofetch < Formula
  desc "A fetch utility for Rust projects"
  homepage "https://github.com/arjav0703/cargofetch"
  url "https://github.com/arjav0703/cargofetch/archive/refs/tags/v1.0.0.tar.gz"
  sha256 ""
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/cargofetch"
  end

  test do
    assert_match "cargofetch 1.0.0", shell_output("#{bin}/cargofetch --version")
  end
end


