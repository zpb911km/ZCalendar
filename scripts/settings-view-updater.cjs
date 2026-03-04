const fs = require('fs');
const path = require('path');

module.exports = {
  readVersion: function (contents) {
    // 从 SettingsView.vue 中读取版本号
    const match = contents.match(/<p><strong>应用版本:<\/strong> ([\d.]+)<\/p>/);
    return match ? match[1] : null;
  },
  writeVersion: function (contents, version) {
    // 生成构建日期
    const buildDate = new Date().toISOString().split('T')[0];
    
    // 更新版本号
    contents = contents.replace(
      /<p><strong>应用版本:<\/strong> [\d.]+<\/p>/,
      `<p><strong>应用版本:</strong> ${version}</p>`
    );
    
    // 更新构建日期
    contents = contents.replace(
      /<p><strong>构建日期:<\/strong> \d{4}-\d{2}-\d{2}<\/p>/,
      `<p><strong>构建日期:</strong> ${buildDate}</p>`
    );
    
    return contents;
  }
};
