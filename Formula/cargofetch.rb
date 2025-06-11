class Cargofetch < Formula
  desc "A fetch utility for Rust projects"
  homepage "https://github.com/arjav0703/cargofetch"
  url "https://github.com/arjav0703/cargofetch/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "2af236d6598ea22491e654565d66d561680a3cd53c59d2d8de2eac35c39a4e7c"
  license "MIT"
  head "https://github.com/arjav0703/cargofetch.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "cargofetch"
  end

  test do
    assert_match "cargofetch 1.0.0", shell_output("#{bin}/cargofetch --version")
  end
end

