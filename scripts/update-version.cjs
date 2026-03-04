const fs = require('fs');
const path = require('path');

// 读取 package.json 获取版本号
const packageJsonPath = path.join(__dirname, '../package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
const version = packageJson.version;

// 生成构建日期（YYYY-MM-DD 格式）
const buildDate = new Date().toISOString().split('T')[0];

// 更新 SettingsView.vue 中的版本号和构建日期
const settingsViewPath = path.join(__dirname, '../src/views/SettingsView.vue');
let settingsViewContent = fs.readFileSync(settingsViewPath, 'utf8');

// 替换版本号和构建日期
settingsViewContent = settingsViewContent.replace(
  /<p><strong>应用版本:<\/strong> [\d.]+<\/p>/,
  `<p><strong>应用版本:</strong> ${version}</p>`
);

settingsViewContent = settingsViewContent.replace(
  /<p><strong>构建日期:<\/strong> \d{4}-\d{2}-\d{2}<\/p>/,
  `<p><strong>构建日期:</strong> ${buildDate}</p>`
);

// 写回文件
fs.writeFileSync(settingsViewPath, settingsViewContent, 'utf8');

console.log(`✓ 版本号已更新为: ${version}`);
console.log(`✓ 构建日期已更新为: ${buildDate}`);
console.log(`✓ 已更新文件: src/views/SettingsView.vue`);