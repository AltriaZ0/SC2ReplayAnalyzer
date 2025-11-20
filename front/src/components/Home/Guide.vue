<!-- src/components/Guide.vue -->
<script setup>
import MessageBox from '../widget/MessageBox.vue';

// 定义消息列表（支持多行字符串、换行等）
const messages = [
  {
    title: '单文件分析：分析单独的.sc2replay文件',
    content: `点击左侧的“单文件分析”，将 .replay 文件拖入分析区域，或者直接从文件浏览器中选择文件，设置好分析选项之后，点击“开始分析”即可进行分析。
解析完成后可查看对局的基本情况、科技升级、建筑建造、单位建造等详细数据。`
  },
  {
    title: '批量分析：分析一整个文件夹的replay文件',
    content: `与单个文件分析类似，只需选择或拖入含有星际2replay文件的文件夹，设置好分析选项之后，点击“开始批量分析”即可。
处理好的信息会显示在页面右侧。`
  },
  {
        title: '我的数据：分析后的结果',
    content: `经由本程序分析的replay数据，可以在这里查看，并能进行查询、筛选等操作。
同时，可以对数据进行备注，可以用于记录战术名称、自己的复盘与心得等信息。    `

  },{

    title: '屏幕提醒',
    content: `是正在开发的小工具，目前还有待完善。可以实现在屏幕顶端设置一个置顶的文字，用于在游戏中进行提醒注卵、砸矿骡、给加速的提醒。`
  }
]
</script>


<template>

    <MessageBox
      v-for="(msg, index) in messages"
      :key="index"
      :title="msg.title"
      :content="msg.content"
    />
</template>


<style scoped>
.content {
  padding: 24px;
  font-size: 16px;
  line-height: 1.5;
  color: #e0e0e0;
}

</style>