<template>
  <div class="page">
    <!-- Toolbar -->
    <header class="toolbar">
      <h1 class="title">屏幕提醒</h1>
      <button class="btn" @click="reset">重置</button>
      <button class="btn" :class="{ 'active': !state.locked }" @click="toggleLock">
        {{ state.locked ? '🔒 已锁定' : '🔓 调整位置' }}
      </button>
      <!-- <button class="btn" @click="saveToLocal">保存</button> -->
      <button class="btn primary" @click="toggleShow">
        {{ state.show ? '隐藏悬浮' : '显示悬浮' }} (Ctrl+Shift+S)
      </button>
    </header>

    <div class="content">
      <section class="left">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 5px;">
          <label class="label" style="margin-bottom: 0;">提醒文字 (支持插入图标)</label>
          <!-- [新增] 图标管理按钮 -->
          <button class="btn-xs" @click="openIconModal">📂 管理/插入图标</button>
        </div>
        <textarea
          ref="textareaRef"
          v-model="state.text"
          class="textarea"
          rows="5"
          placeholder="在这里输入提醒文字…&#10;例如：12:00 [icon1] 出动。"
          @blur="updateCursorPos"
          @click="updateCursorPos"
          @keyup="updateCursorPos"
        />

        <div class="grid-2">
          <div>
            <label class="label">字体</label>
            <select v-model="state.font.family" class="input">
              <option value="Inter, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, 'Apple Color Emoji', 'Segoe UI Emoji'">系统默认</option>
              <option value="'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace">等宽（JetBrains Mono）</option>
              <option value="'Noto Sans SC', 'Microsoft YaHei', 'PingFang SC', system-ui, sans-serif">思源黑体 / 微软雅黑</option>
            </select>
          </div>
          <div>
            <label class="label">粗细</label>
            <select v-model.number="state.font.weight" class="input">
              <option :value="400">正常</option>
              <option :value="600">加粗</option>
            </select>
          </div>
        </div>

        <div class="grid-3">
          <div>
            <label class="label">字号：{{ state.font.size }} px</label>
            <input class="range" type="range" min="12" max="72" v-model.number="state.font.size" />
          </div>
          <div>
            <label class="label">字距：{{ state.style.letterSpacing.toFixed(1) }} px</label>
            <input class="range" type="range" min="-1" max="6" step="0.1" v-model.number="state.style.letterSpacing" />
          </div>
          <div>
            <label class="label">行高：{{ state.style.lineHeight.toFixed(2) }}</label>
            <input class="range" type="range" min="1" max="2" step="0.05" v-model.number="state.style.lineHeight" />
          </div>
        </div>

        <div class="grid-3">
          <div>
            <label class="label">文本颜色</label>
            <input class="input" type="color" v-model="state.colors.text" />
          </div>
          <div>
            <label class="label">背景颜色</label>
            <input class="input" type="color" v-model="state.colors.bg" />
          </div>
          <div>
            <label class="label">背景不透明度：{{ state.style.bgOpacity }}</label>
            <input class="range" type="range" min="0" max="1" step="0.05" v-model.number="state.style.bgOpacity" />
          </div>
        </div>

        <div class="grid-3">
          <div>
            <label class="label">顶部内边距：{{ state.style.paddingY }} px</label>
            <input class="range" type="range" min="4" max="32" step="1" v-model.number="state.style.paddingY" />
          </div>
          <div>
            <label class="label">左右内边距：{{ state.style.paddingX }} px</label>
            <input class="range" type="range" min="8" max="64" step="2" v-model.number="state.style.paddingX" />
          </div>
          <div>
            <label class="label">阴影</label>
            <select v-model="state.style.shadow" class="input">
              <option value="none">无</option>
              <option value="sm">浅</option>
              <option value="md">中</option>
              <option value="lg">重</option>
            </select>
          </div>
        </div>
<!-- 
        <div class="grid-3" style="margin-top: 12px;">
          <div>
            <label class="label">窗口宽度（px）</label>
            <input class="input" type="number" min="400" max="9999" v-model.number="state.window.width" />
          </div>
          <div>
            <label class="label">窗口高度（px）</label>
            <input class="input" type="number" min="40" max="600" v-model.number="state.window.height" />
          </div>
          <div>
            <label class="label">宽度模式</label>
            <select v-model="state.layout.width" class="input">
              <option value="full">全宽</option>
              <option value="center">居中（最大 960px）</option>
            </select>
          </div>
        </div> -->

        <!-- <div class="grid-3" style="margin-top: 12px;">
          <div>
            <label class="label">层级 z-index</label>
            <input class="input" type="number" v-model.number="state.layout.zIndex" min="10" max="999999" />
          </div>
          <div>
            <label class="label">交互</label>
            <select v-model="state.layout.pointer" class="input">
              <option value="auto">可点击</option>
              <option value="none">穿透（不拦截鼠标）</option>
            </select>
          </div>
          <div>
            <label class="label">启用 N 秒后闪烁</label>
            <select v-model="state.behavior.blinkEnabled" class="input">
              <option :value="true">启用</option>
              <option :value="false">禁用</option>
            </select>
          </div>
        </div>

        <div class="grid-3" style="margin-top: 12px;">
          <div>
            <label class="label">N 秒后开始闪烁：{{ state.behavior.blinkAfter }} s</label>
            <input class="range" type="range" min="10" max="600" step="5" v-model.number="state.behavior.blinkAfter" />
          </div>
          <div>
            <label class="label">（可选）闪烁持续秒数：{{ state.behavior.blinkDuration }} s</label>
            <input class="range" type="range" min="5" max="120" step="5" v-model.number="state.behavior.blinkDuration" />
          </div>
          <div style="display:flex;align-items:flex-end;">
            <span class="label">
              全局快捷键：
              <code>`</code> 重置闪烁倒计时
            </span>
          </div>
        </div> -->

        <!-- <div class="tips">
          快捷键：
          <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>S</kbd> 显示/隐藏；
          <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>↑/↓</kbd> 调整字号；
          <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>C</kbd> 复制当前文字；
          <code>`</code>（全局）重置即将闪烁的时间。
        </div> -->

          <div style="margin-top: 20px; font-size: 12px; color: #666; line-height: 1.5;">
            提示：<br/>
            1. 只有点击上方 <b>"解锁位置"</b> 后，悬浮窗才可以被鼠标选中并拖动。<br/>
            2. 锁定后，鼠标会穿透悬浮窗（点击穿透），不影响你操作背后的内容。<br/>
            3. 使用 <code>[标签名]</code> 插入图标，标签名在弹窗中可修改。
         </div>
      </section>

      <section class="right">
        <div class="preview-title">预览</div>
        <div class="preview">
          <!-- [修改] 改用 v-html 渲染图片 -->
          <div class="banner" :style="bannerStyle" v-html="parsedHtmlText"></div>
        </div>
      </section>
    </div>

    <!-- [新增] 图标管理模态框 -->
    <div v-if="showIconModal" class="modal-overlay" @click.self="showIconModal = false">
      <div class="modal-content">
        <div class="modal-header">
          <h3>自定义图标库</h3>
          <div style="display:flex; gap: 10px;">
            <input type="file" ref="fileInputRef" accept="image/*" multiple style="display: none" @change="handleFileUpload" />
            <button class="btn primary" @click="triggerFileUpload">📤 上传图标</button>
            <button class="close-btn" @click="showIconModal = false">×</button>
          </div>
        </div>
        
        <div class="modal-body">
          <div v-if="userIcons.length === 0" class="empty-state">
            暂无图标，请点击右上角上传图片。
          </div>
          <div v-else class="icon-grid">
            <div v-for="(icon, index) in userIcons" :key="index" class="icon-card">
              <div class="img-wrapper" @click="insertIcon(icon.name)" title="点击插入">
                <img :src="icon.src" loading="lazy" />
              </div>
              <div class="edit-wrapper">
                <span class="bracket">[</span>
                <input v-model="icon.name" class="name-input" @change="saveIconsToStorage" />
                <span class="bracket">]</span>
                <button class="delete-btn" @click="deleteIcon(index)" title="删除">×</button>
              </div>
            </div>
          </div>
        </div>
        <div class="modal-footer">已存储 {{ userIcons.length }} 个图标 (本地缓存)</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, computed, onMounted, onUnmounted, watch, ref } from 'vue' // [修改] 引入 ref
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { LogicalSize } from '@tauri-apps/api/window'
import {listen, emit } from '@tauri-apps/api/event'
import { register as registerShortcut, unregisterAll } from '@tauri-apps/plugin-global-shortcut'

const BANNER_LABEL = 'screen-banner'
let bannerWin: WebviewWindow | null = null

// ---------------- [新增] 自定义图标逻辑 ----------------
interface UserIcon { name: string; src: string; }
const showIconModal = ref(false)
const userIcons = ref<UserIcon[]>([])
const fileInputRef = ref<HTMLInputElement | null>(null)
const textareaRef = ref<HTMLTextAreaElement | null>(null) // [新增]
const cursorPosition = ref(0) // [新增]

function openIconModal() { showIconModal.value = true }
function triggerFileUpload() { fileInputRef.value?.click() }

async function handleFileUpload(event: Event) {
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files || files.length === 0) return
  for (let i = 0; i < files.length; i++) {
    const file = files[i]
    let safeName = file.name.split('.')[0].toLowerCase().replace(/\s+/g, '_')
    let counter = 1; let tempName = safeName
    while (userIcons.value.some(icon => icon.name === tempName)) { tempName = `${safeName}_${counter}`; counter++ }
    const base64 = await toBase64(file)
    userIcons.value.push({ name: tempName, src: base64 as string })
  }
  saveIconsToStorage()
  target.value = ''
}

function toBase64(file: File) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.readAsDataURL(file)
    reader.onload = () => resolve(reader.result)
    reader.onerror = error => reject(error)
  })
}

function deleteIcon(index: number) {
  if (confirm('确定删除?')) { userIcons.value.splice(index, 1); saveIconsToStorage() }
}

const ICONS_STORE_KEY = 'user-custom-icons'
function saveIconsToStorage() {
  try { localStorage.setItem(ICONS_STORE_KEY, JSON.stringify(userIcons.value)) } catch (e) { alert('存储空间不足'); }
}
function loadIconsFromStorage() {
  const raw = localStorage.getItem(ICONS_STORE_KEY)
  if (raw) { try { userIcons.value = JSON.parse(raw) } catch (e) { console.error(e) } }
}

function updateCursorPos() {
  if (textareaRef.value) cursorPosition.value = textareaRef.value.selectionStart
}

function insertIcon(tagName: string) {
  const insertText = `[${tagName}]`
  const originalText = state.text || ''
  const p = cursorPosition.value
  state.text = originalText.slice(0, p) + insertText + originalText.slice(p)
  cursorPosition.value += insertText.length
  showIconModal.value = false
  setTimeout(() => { textareaRef.value?.focus(); textareaRef.value?.setSelectionRange(cursorPosition.value, cursorPosition.value) }, 100)
}

function escapeHtml(text: string) {
  return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;").replace(/'/g, "&#039;")
}

// [新增] HTML 解析 computed
const parsedHtmlText = computed(() => {
  if (!state.text) return '示例提示文字'
  const safeText = escapeHtml(state.text)
  return safeText.replace(/\[([a-zA-Z0-9_\-\u4e00-\u9fa5\s]+)\]/g, (match, key) => {
    const cleanKey = key.trim()
    const icon = userIcons.value.find(i => i.name === cleanKey)
    if (icon) {
      return `<img src="${icon.src}" style="height: 1.2em; vertical-align: text-bottom; margin: 0 1px;" alt="${cleanKey}"/>`
    }
    return match
  })
})

// ---------------- 状态 (保持原有) ----------------
const state = reactive({
  text: '',
  show: false,
  locked:true,
  font: {
    size: 28,
    family: "Inter, system-ui, -apple-system, 'Segoe UI', Roboto, Helvetica, Arial",
    weight: 700
  },
  colors: { text: '#ffffff', bg: '#111827' },
  style: {
    letterSpacing: 0.2,
    lineHeight: 1.2,
    paddingX: 24,
    paddingY: 10,
    bgOpacity: 0.85,
    shadow: 'md' as 'none' | 'sm' | 'md' | 'lg',
  },
  layout: {
    width: 'full' as 'full' | 'center',
    zIndex: 99999,
    pointer: 'none' as 'none' | 'auto',
  },
  window: {
    width: 1920,
    height: 60,
  },
  behavior: {
    blinkEnabled: true,
    blinkAfter: 180,      // N 秒后开始闪烁
    blinkDuration: 30,    // 闪烁持续时间
  },
})

// 样式（发给预览 + 子窗口）
const bannerStyle = computed(() => ({
  color: state.colors.text,
  background: hexToRgba(state.colors.bg, state.style.bgOpacity),
  fontSize: state.font.size + 'px',
  fontFamily: state.font.family,
  fontWeight: String(state.font.weight),
  letterSpacing: state.style.letterSpacing + 'px',
  lineHeight: String(state.style.lineHeight),
  padding: `${state.style.paddingY}px ${state.style.paddingX}px`,
  boxShadow: shadowMap[state.style.shadow],
  whiteSpace: 'pre-wrap' as const, // [修改] 强制换行策略以支持 v-html
  maxWidth: '1200px',
  width: 'fit-content',
  overflow: 'hidden',
  textOverflow: 'ellipsis',
  cursor: (state.locked ? 'default' : 'move') as 'default' | 'move', // [修复] TS 类型断言
  pointerEvents: (state.locked ? 'none' : 'auto') as 'none' | 'auto',
}))

const bannerWrapStyle = computed(() => ({
  position: 'fixed',
  top: '0', left: '0', right: '0',
  zIndex: String(state.layout.zIndex),
  pointerEvents: state.layout.pointer,
  display: 'flex',
  justifyContent: state.layout.width === 'full' ? 'stretch' : 'center',
}))

async function toggleLock() {
  state.locked = !state.locked
  await updateLockState()
  emitUpdate() // 通知子窗口更新样式
}

async function updateLockState() {
  if (!bannerWin) return
  // true = 忽略鼠标（穿透/锁定），false = 捕获鼠标（不穿透/可拖动）
  await bannerWin.setIgnoreCursorEvents(state.locked)
  if (!state.locked) {
    await bannerWin.setFocus() // 解锁时聚焦，方便操作
  }
}

function emitUpdate() {
  emit('screen-banner:update', {
    text: parsedHtmlText.value, // [修改] 发送解析后的 HTML
    style: bannerStyle.value,
    layout: {
      zIndex: state.layout.zIndex,
      pointer: state.layout.pointer,
      // pointer: state.locked ? 'none' : 'auto', // [修改] 确保逻辑一致
      widthMode: state.layout.width,
    },
    // window: {
    //   width: state.window.width,
    //   height: state.window.height,
    // },
    behavior: {
      blinkEnabled: state.behavior.blinkEnabled,
      blinkAfter: state.behavior.blinkAfter,
      blinkDuration: state.behavior.blinkDuration,
    },
    locked: state.locked
  })
  console.log('emitUpdate')
}

// ---------------- 窗口管理 ----------------
async function openBannerWindow() {
  if (bannerWin) {
    try {
      await bannerWin.show()
      return
    } catch {
      bannerWin = null
    }
  }

  bannerWin = new WebviewWindow(BANNER_LABEL, {
    url: '/floating-banner',
    // width: state.window.width,
    // height: state.window.height,
    width: 400,
    height: 100,
    x: 0,
    y: 0,
    decorations: false,
    transparent: true,
    alwaysOnTop: true,
    resizable: false,
    focus: false,
    shadow: false
  })
  
  bannerWin.once('tauri://created', async () => {
    try {
      await updateLockState()
      // await bannerWin?.setIgnoreCursorEvents(true)
    } catch (e) {
      console.warn('setIgnoreCursorEvents failed', e)
    }
  })

  bannerWin.once('tauri://destroyed', () => {
    bannerWin = null
    state.show = false
  })

}

async function closeBannerWindow() {
  if (!bannerWin) return
  try {
    await bannerWin.close()
  } catch (err) {
    console.warn('close banner failed', err)
  } finally {
    bannerWin = null
  }
}

async function toggleShow() {
  state.show = !state.show

  if (state.show) {
    saveToLocal()
    await openBannerWindow()
    // 打开后立刻发一次当前状态
      setTimeout(() => {
        emitUpdate()
    }, 1000) 
  } else {
    await closeBannerWindow()

  }
}

// 窗口尺寸变化时，同步给子窗口（如果已存在）
// watch(
//   () => [state.window.width, state.window.height],
//   ([w, h]) => {
//     if (bannerWin) {
//       bannerWin.setSize(new LogicalSize(w, h))
//       emitUpdate()
//     }
//   }
// )

// ---------------- 工具 ----------------
const shadowMap: Record<string, string> = {
  none: 'none',
  sm: '0 1px 2px rgba(0,0,0,.25)',
  md: '0 6px 16px rgba(0,0,0,.35)',
  lg: '0 14px 28px rgba(0,0,0,.45)'
}

function hexToRgba(hex: string, a: number) {
  const m = hex.replace('#','')
  const full = m.length === 3 ? m.split('').map(ch => ch + ch).join('') : m
  const bigint = parseInt(full, 16)
  const r = (bigint >> 16) & 255
  const g = (bigint >> 8) & 255
  const b = bigint & 255
  return `rgba(${r}, ${g}, ${b}, ${a})`
}

function reset() {
  state.text = ''
  state.font.size = 28
  state.font.family = "Inter, system-ui, -apple-system, 'Segoe UI', Roboto, Helvetica, Arial"
  state.font.weight = 700
  state.colors.text = '#ffffff'
  state.colors.bg = '#111827'
  state.style.letterSpacing = 0.2
  state.style.lineHeight = 1.2
  state.style.paddingX = 24
  state.style.paddingY = 10
  state.style.bgOpacity = 0.85
  state.style.shadow = 'md'
  state.layout.width = 'full'
  state.layout.zIndex = 99999
  state.layout.pointer = 'none'
  // state.window.width = 1920
  // state.window.height = 60
  state.behavior.blinkEnabled = true
  state.behavior.blinkAfter = 180
  state.behavior.blinkDuration = 30
  state.locked = true // [新增]
  updateLockState()   // [新增]
}

// 持久化（localStorage）
const STORE_KEY = 'screen-reminder-v1'
function saveToLocal() {
  localStorage.setItem(STORE_KEY, JSON.stringify(state))
}

function loadFromLocal() {
  try {
    const raw = localStorage.getItem(STORE_KEY)
    if (raw) {
      const parsed = JSON.parse(raw)
      // 逐个属性合并，防止整个 state 代理对象被破坏
      if (parsed.text !== undefined) state.text = parsed.text
      if (parsed.font) Object.assign(state.font, parsed.font)
      if (parsed.colors) Object.assign(state.colors, parsed.colors)
      if (parsed.style) Object.assign(state.style, parsed.style)
      if (parsed.layout) Object.assign(state.layout, parsed.layout)
      if (parsed.window) Object.assign(state.window, parsed.window)
      if (parsed.behavior) Object.assign(state.behavior, parsed.behavior)
      // 恢复 show 状态为 false，防止一打开APP就自动弹窗（如果这是你期望的）
      state.show = false 
    }
  } catch (e) {
    console.error('Load failed', e)
  }
}

// 快捷键（窗口内）
function onKey(e: KeyboardEvent) {
  if (!e.ctrlKey || !e.shiftKey) {
    // 单独按 ` 时，在当前窗口也重置闪烁时间
    if (e.code === 'Backquote') {
      emit('screen-banner:resetBlink')
    }
    return
  }
  if (e.code === 'KeyS') { e.preventDefault(); toggleShow() }
  if (e.code === 'ArrowUp') { e.preventDefault(); state.font.size = Math.min(96, state.font.size + 1) }
  if (e.code === 'ArrowDown') { e.preventDefault(); state.font.size = Math.max(10, state.font.size - 1) }
  if (e.code === 'KeyC') { e.preventDefault(); navigator.clipboard.writeText(state.text || '') }
}

// 生命周期
onMounted(async () => {
  loadFromLocal()
  loadIconsFromStorage() // [新增] 加载自定义图标
  window.addEventListener('keydown', onKey)

  // 全局快捷键：在 Windows 任意界面按 `，重置闪烁倒计时
  // try {
  //   await registerShortcut('`', (event) => {
  //     console.log('global ` pressed in floating window')
  //     if (event.state === 'Pressed') {
  //       emit('screen-banner:resetBlink')
  //     }
  //   })
  // } catch (err) {
  //   console.warn('register global shortcut failed', err)
  // }

  await listen('screen-banner:ready', () => {
    console.log('Floating banner is ready, syncing state...')
    emitUpdate()
  })

})

onUnmounted(() => {
  window.removeEventListener('keydown', onKey)
  unregisterAll().catch(() => {})
})

// 自动保存 + 同步到子窗口
watch(
  state,
  () => {
    saveToLocal()
    emitUpdate()
  },
  { deep: true }
)
</script>

<style scoped>
.page { display: flex; flex-direction: column; height: 100%; color: #e5e7eb; background: #0f1113; }
.toolbar h1 { font-size: 18px; font-weight: 700; }
.spacer { flex: 1; }
.title{ font-size:18px; margin:4px 4px; font-weight:700; }
.content {
  display:grid;
  grid-template-columns: 420px 1fr;
  gap:16px;
  padding:16px 20px;
  height: calc(100vh - 50px);
  margin-top: 50px;
  box-sizing:border-box;
  overflow:hidden;
}
.left, .right {
  background: #161a1f;
  border: 1px solid #23272e;
  border-radius: 14px;
  padding: 14px;
  overflow: auto;
}

.label { font-size: 12px; color: #a6adbb; margin-bottom: 6px; display: block; }
.input, .textarea, .range { width: 100%; box-sizing: border-box; }
.input, .textarea {
  background: #0f1317;
  color: #e5e7eb;
  border: 1px solid #2a2f36;
  border-radius: 10px;
  padding: 10px 12px;
}
.textarea { resize: vertical; }
.range { accent-color: #3b82f6; }
.grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin-top: 12px; }
.grid-3 { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 10px; margin-top: 12px; }

.btn {
  border: 1px solid #2a2f36;
  background: #0f1317;
  color: #e5e7eb;
  padding: 4px 12px;
  border-radius: 10px;
  cursor: pointer;
}
.btn:hover { filter: brightness(1.1); }
.btn.primary { background: #3b82f6; border-color: transparent; color: white; }

.preview-title { font-size: 12px; color: #a6adbb; margin-bottom: 8px; }
.preview { border: 1px dashed #2a2f36; border-radius: 12px; padding: 10px; background: #0f1113; }
.banner { width: 100%; text-align: left; border-radius: 10px; }

/* 悬浮条容器（如果以后用 teleport） */
.floating-banner { position: fixed; top: 0; left: 0; right: 0; }

code { font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; }

/* ---------------- [新增] 模态框样式 ---------------- */
.btn-xs {
  border: 1px solid #3b82f6;
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
  padding: 2px 8px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 11px;
}
.btn-xs:hover { background: rgba(59, 130, 246, 0.2); }

.modal-overlay {
  position: fixed; top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0,0,0,0.7);
  display: flex; justify-content: center; align-items: center;
  z-index: 10000;
}
.modal-content {
  background: #1f242d;
  width: 700px;
  max-height: 80vh;
  border-radius: 12px;
  border: 1px solid #374151;
  display: flex; flex-direction: column;
}
.modal-header {
  padding: 16px;
  border-bottom: 1px solid #374151;
  display: flex; justify-content: space-between; align-items: center;
}
.modal-body {
  padding: 20px;
  overflow-y: auto;
  flex: 1;
}
.modal-footer {
  padding: 10px 20px;
  border-top: 1px solid #374151;
  font-size: 11px;
  color: #6b7280;
  text-align: right;
}
.close-btn { background: none; border: none; color: #9ca3af; cursor: pointer; font-size: 20px; }
.empty-state { text-align: center; color: #6b7280; padding: 40px; }

.icon-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 16px;
}
.icon-card {
  background: #262b36;
  border: 1px solid #374151;
  border-radius: 8px;
  padding: 8px;
  display: flex;
  flex-direction: column;
  align-items: center;
}
.icon-card:hover { border-color: #4b5563; }

.img-wrapper {
  width: 100%;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  background: #161a1f;
  border-radius: 4px;
  margin-bottom: 8px;
}
.img-wrapper img { max-width: 100%; max-height: 100%; object-fit: contain; }

.edit-wrapper {
  display: flex;
  align-items: center;
  width: 100%;
  font-size: 12px;
  color: #9ca3af;
}
.bracket { opacity: 0.5; font-family: monospace; }
.name-input {
  background: transparent;
  border: none;
  border-bottom: 1px dashed #4b5563;
  color: #e5e7eb;
  font-family: monospace;
  width: 100%;
  text-align: center;
  margin: 0 4px;
  font-size: 11px;
}
.name-input:focus { outline: none; border-bottom: 1px solid #3b82f6; color: #3b82f6; }
.delete-btn {
  background: none; border: none; color: #ef4444; cursor: pointer; font-size: 16px;
  opacity: 0.5; margin-left: 2px;
}
.delete-btn:hover { opacity: 1; }
</style>