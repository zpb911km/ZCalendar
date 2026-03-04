module.exports = {
  readVersion: function (contents) {
    // 读取 Cargo.toml 中的版本号
    const match = contents.match(/^version\s*=\s*"([^"]+)"/m);
    return match ? match[1] : null;
  },
  writeVersion: function (contents, version) {
    // 更新 Cargo.toml 中的版本号
    return contents.replace(
      /^version\s*=\s*"([^"]+)"/m,
      `version = "${version}"`
    );
  }
};
