#!/usr/bin/env node

// 使用标准的 napi-rs 生成的文件
const { generateProject, listTemplates, getTemplateInfo } = require('../dist/index.js');

console.log('🦀 Testing napi-rs generated bindings...\n');

try {
  // 测试 1: 列出模板
  console.log('📋 Testing listTemplates("vue"):');
  const vueTemplates = listTemplates('vue');
  console.log('Result:', vueTemplates);
  console.log('');

  // 测试 2: 获取模板信息
  console.log('📝 Testing getTemplateInfo("vue", "basic"):');
  const templateInfo = getTemplateInfo('vue', 'basic');
  console.log('Result:', templateInfo);
  console.log('');

  // 测试 3: 生成项目
  console.log('🏗️ Testing generateProject:');
  const result = generateProject({
    name: 'my-napi-project',
    projectType: 'vue',
    template: 'basic'
  });
  console.log('Result:', result);
  console.log('');

  console.log('✅ All napi-rs tests passed!');
} catch (error) {
  console.error('❌ Test failed:', error);
  process.exit(1);
}