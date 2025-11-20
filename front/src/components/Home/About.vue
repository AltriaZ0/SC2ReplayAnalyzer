<!-- src/components/About.vue -->
<template>
  <div class="content">
    <div v-html="marked(aboutMd)"></div>
  </div>
</template>

<script setup>
import { marked } from 'marked'
const aboutMd = `

  # 程序介绍
  SC2ReplayAnalyzer是一款免费的能够应用于星际争霸2战术学习、游戏后复盘、数据统计等方面的小程序。

  本程序前端页面使用Tauri编写，分析内核使用python，通过sc2reader库获取replay event。



  # 欢迎联系
  如果在使用过程中遇到debug或者有任何建议，欢迎联系作者：

  知乎ID：赞君，
  
  bilibili：小赞小赞，
  
  github：AltriaZ0
  
  软件交流 & 后续更新版本的下载 & sc2交流，欢迎加入Q群924040644

  如果您有发现bug，或者有任何意见与建议，欢迎在各平台私信告诉我，非常感谢！
  
  # 开源与参与贡献

  本项目现已开源，Github仓库地址 https://github.com/AltriaZ0/SC2ReplayAnalyzer"

  如果有本项目感兴趣的小伙伴想要加入开发团队，欢迎与我联系~
`
</script>

<style scoped>
.content {
  background-color: #1a1a1a;
  border-radius: 8px;
  padding: 16px;
  margin: 0px 0px;
  color: #e0e0e0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}
</style>