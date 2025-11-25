<template>
  <div 
    ref="containerRef"
    class="banner-container" 
    :style="containerStyle"
    data-tauri-drag-region
  >
    {{ text || '示例提示文字' }}
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { listen,emit} from '@tauri-apps/api/event'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'

// 显示文字 & 样式
const text = ref('')
const bannerStyle = ref<Record<string, any>>({})

const containerStyle = ref<Record<string, any>>({})
const containerRef = ref<HTMLElement | null>(null)

// const blinking = ref(false)
// let timerId: number | null = null
// let stopId: number | null = null
// let blinkAfter = 180
// let blinkDuration = 30
// let blinkEnabled = true

let unlisten: (() => void) | null = null
let resizeObserver: ResizeObserver | null = null
// function resetTimer() {
//   blinking.value = false
//   if (timerId) clearTimeout(timerId)
//   if (stopId) clearTimeout(stopId)

//   if (!blinkEnabled) return

//   timerId = window.setTimeout(() => {
//     blinking.value = true
//     if (blinkDuration > 0) {
//       stopId = window.setTimeout(() => {
//         blinking.value = false
//       }, blinkDuration * 1000)
//     }
//   }, blinkAfter * 1000)
// }

const updateWindowSize = async () => {
  await nextTick()
  const el = containerRef.value
  if (!el) return

  // 获取实际渲染宽度
  // +2 防止某些 DPI 下边缘裁切
  const width = el.offsetWidth + 2
  const height = el.offsetHeight + 2

  const appWindow = getCurrentWindow()
  await appWindow.setSize(new LogicalSize(width, height))
}

onMounted(async () => {
  // 监听从主窗口发来的事件
  const appWindow = getCurrentWindow()

  unlisten = await listen('screen-banner:update', async(event) => {
    const payload = event.payload as { text: string; style: Record<string, any>; behavior: Record<string, any> }
    text.value = payload.text
    containerStyle.value = {
      ...payload.style,

      width: 'fit-content', 
      height: 'fit-content',
      // 强制最大宽度
      maxWidth: '1200px',
      // 溢出直接裁切
      whiteSpace: 'pre', 
      
      margin: '0',
      pointerEvents: payload.locked ? 'none' : 'auto'
    }
      await appWindow.setIgnoreCursorEvents(payload.locked)
    if (!payload.locked) {
      await appWindow.setFocus()
    }
    // bannerStyle.value = payload.style
    // if (payload.behavior) {
    // blinkEnabled = !!payload.behavior.blinkEnabled
    // blinkAfter = payload.behavior.blinkAfter ?? blinkAfter
    // blinkDuration = payload.behavior.blinkDuration ?? blinkDuration
    // }
    // resetTimer()
  })

  // const un2 = await listen('screen-banner:resetBlink', () => {
  //   resetTimer()
  // })
 if (containerRef.value) {
    resizeObserver = new ResizeObserver(() => {
      updateWindowSize()
    })
    resizeObserver.observe(containerRef.value)
  }

  await emit('screen-banner:ready')
})

onBeforeUnmount(() => {
  unlisten && unlisten()
  if (resizeObserver) resizeObserver.disconnect()
})
</script>

<style>
:global(body) {
  margin: 0;
  padding: 0;
  background: transparent !important; 
  overflow: hidden;
}

.banner-container {
  width: fit-content;
  text-align: left; 
  border-radius: 10px;
  user-select: none; 
  cursor: default;
  min-width: 0; 
}
/* .blinking {
  animation: blink 0.8s linear infinite;
}
@keyframes blink {
  0%, 50%, 100% { opacity: 1; }
  25%, 75% { opacity: 0.2; }
} */
</style>
