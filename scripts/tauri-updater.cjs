module.exports = {
  readVersion: function (contents) {
    // 读取 JSON 中的版本号
    const json = JSON.parse(contents);
    return json.version;
  },
  writeVersion: function (contents, version) {
    // 更新 JSON 中的版本号
    const json = JSON.parse(contents);
    json.version = version;
    return JSON.stringify(json, null, 2);
  }
};
