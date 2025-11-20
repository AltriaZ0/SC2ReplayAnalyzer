  <template>
    <div class="page">
      <!-- 顶部工具条 -->
      <header class="toolbar">
        <div class="left">
          <h1 class="title">批量分析</h1>
        </div>
      </header>

      <div class="content">
        <!-- 左：上传与选项 -->
        <section class="left-pane">
          <div
            class="dropzone"
            :class="{ dragover }"
            @dragover.prevent="dragover = true"
            @dragleave.prevent="dragover = false"
            @drop.prevent="onDrop"
            v-if="!fileMeta"
          >
            <div class="dz-inner">
              <svg class="dz-icon" viewBox="0 0 24 24" aria-hidden="true">
                <path fill="currentColor" d="M16.59 5.59 12 1 7.41 5.59 8.83 7l2.17-2.17V14h2V4.83L15.17 7 16.59 5.59zM19 10h-2v6H7v-6H5v6a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-6z"/>
              </svg>
              <p class="dz-title">拖拽含有<span>.SC2Replay文件</span> 的文件夹到此处</p>
              <p class="dz-sub">或 <button class="link" @click="pickFile">点击选择</button> 本地文件夹</p>
              <input ref="fileInput" class="hidden" type="file" accept=".SC2Replay,.sc2replay" @change="onFile" />
            </div>
          </div>


          <div class="card" v-if="fileMeta">
            
            <div class="card-header">
              <div class="file-name">文件夹信息</div>
              <button class="btn tiny" @click="clearFile">移除</button>
            </div>
            <ul class="meta">
              <li><span>名称</span><span>{{ fileMeta.name }}              </span></li>
              <li><span>Replay文件数量</span> <span>{{fileMeta.count}}</span></li>
              <!-- <li><span>修改时间</span><span>{{ fileMeta.mtime }}</span></li>
              <li><span>创建时间</span><span>{{ fileMeta.ctime }}          </span></li> -->
            </ul>
          </div>

          <div class="card">
            <div class="card-header">
              <div class="card-title">分析选项</div>
              <button class="btn ghost tiny" @click="resetOptions">重置</button>
            </div>
            <div class="options">
              <label class="opt"><input type="checkbox" v-model="opts.fulltime"/>统计全部时间（即使不发生建造）</label>
              <!-- <label class="opt"><input type="checkbox" v-model="opts.cancel"/> 提醒建造后取消的信息 </label> -->
              <label class="opt"><input type="checkbox" v-model="opts.buildList"/> 建筑建造统计 </label>
              <label class="opt"><input type="checkbox" v-model="opts.upgradeList"/> 科技升级统计 </label>
              <label class="opt"><input type="checkbox" v-model="opts.UnitList"/> 单位建造统计 </label>
              <!-- <label class="opt"><input type="checkbox" v-model="opts.terranFly"/> 不统计Terran的建筑移动 </label> -->
              <!-- <label class="opt"><input type="checkbox" v-model="opts.workerNumber"/> 农民数量统计(demo) </label> -->
              <!-- <label class="opt"><input type="checkbox" v-model="opts.exportXlsx"/> 导出 Excel（.xlsx）</label> -->
              <!-- <label class="opt"><input type="checkbox" v-model="opts.exportTxt"/> 导出 txt（旧功能）</label> -->
              <label class="opt"><input type="checkbox" v-model="opts.exportSummary"/> 创建批量分析汇总表格</label> 
            </div>

            <div class="actions">
              <button class="btn primary" :disabled="!fileMeta || busy" @click="startAnalyze">
                {{ busy ? '分析中…' : '开始分析' }}
              </button>
              <button class="btn success" :disabled="!fileMeta  || !result || busy" @click="openExportDir">打开导出目录</button>
              <button v-if="result && opts.exportSummary"  class="btn success" :disabled="!fileMeta  || !result || busy" @click="openSummary">打开汇总表格目录</button>
              
            </div>

          <div class="progress-wrapper" v-if="busy">
             <div class="progress-info">
               <span class="status-text">{{ progressMsg }}</span>
               <span class="status-percent">{{ progress.toFixed(0) }}%</span>
             </div>
            <div class="progress">
              <div class="bar" :style="{ width: progress + '%' }"></div>
            </div>
          </div>
          </div>
        </section>

        <!-- 右：结果展示 -->
        <section class="right-pane"> 
          <div v-if="processedReplays.length === 0" class="placeholder muted">
            等待开始分析...<br>
            分析结果将实时显示在这里
          </div>

          <div class="result-list" v-else>
            <!-- 使用 transition-group 增加列表动画 -->
            <transition-group name="list">
              <div v-for="(item, index) in processedReplays" :key="item.fileName + index" class="replay-card-wrapper">
                
                <!-- 单个录像卡片 -->
                <div class="card replay-card">
                  <!-- 卡片头部：地图、对阵、操作 -->
                  <div class="card-header rep-header">
                    <div class="header-left">
                      <span class="rep-index">#{{ processedReplays.length - index }}</span>
                      <span class="rep-map"> {{ item.data.map_name }} </span>
                      <!-- <span class="rep-vs badge"> {{ item.data.raceBattle }} </span> -->
                      <span class="rep-time muted">{{ formatDuration(item.data.duration) }}</span>
                    </div>
                    <div class="header-right">
                      <!-- 单个录像的打开文件夹按钮 -->
                      <button class="btn tiny ghost" @click="openPathSafe(item.data.output)">
                        📂 打开文件夹
                      </button>
                    </div>
                  </div>

                  <!-- 玩家列表与建造流程 -->
                  <div class="players-container">
                    <div v-for="(pInfo, pName) in item.data.playersInfo" :key="pName" class="player-block">
                        <div class="player-title">
                          <span class="p-name">{{ pName }}<span class="p-race">({{ pInfo.race }})</span></span>
                          <b :class="{ win: pInfo.result, loss: !pInfo.result }">
                            {{ pInfo.result ? '胜利' : '失败' }}
                          </b>
                        </div>
                        <!-- 复用 BuildBox 组件 -->
                        <BuildBox 
                          :title="pName" 
                          :race="pInfo.race" 
                          :outputPath="pInfo.outputPath" 
                          :result="pInfo.result" 
                          :content="pInfo.buildOrder"
                          class="mini-build-box" 
                        />
                    </div>
                  </div>
                </div>

              </div>
            </transition-group>
          </div>
        </section>
      </div>

    </div>
  </template>

  <script setup lang="ts">
  import { ref, reactive, computed, onMounted, onBeforeUnmount } from 'vue'
  import { invoke } from '@tauri-apps/api/core'
  import { listen, UnlistenFn } from '@tauri-apps/api/event'
  import { open } from '@tauri-apps/plugin-dialog'
  import { appDataDir, dirname, join } from '@tauri-apps/api/path'
  import { openPath } from '@tauri-apps/plugin-opener';
  import { stat, type FileInfo, readDir } from '@tauri-apps/plugin-fs'
  import BuildBox from '../single/BuildBox.vue'
// 对应 Python: playersInfo 下的具体结构
interface BoStep {
  t: string
  action: string
}

interface PlayerInfo {
  race: string
  result: boolean
  buildOrder: BoStep[]
  outputPath: string
}

// 对应 Python: repInfo
interface ReplayInfo {
  map_name: string
  ladder?: boolean
  duration: number
  playersInfo: Record<string, PlayerInfo>
  winner: string
  region: string
  endTime?: string
  raceBattle: string
  output: string // 录像的输出文件夹路径
}


  interface ProgressPayload {
  percentage: number
  message: string
  replayName?: string
  replayPath?: string
  replayInfo?: ReplayInfo // 只有当分析完一个录像时，这个字段才会有值
}


  interface AnalyzeResult {
    success: boolean;
    outputPath?: string;
    summary?: any;
  }

  interface ProcessedReplayItem {
  fileName: string
  data: ReplayInfo
}


  interface FileMeta {
    name: string
    count: number
    ctime: string
    mtime: string
    path: string
  }
  const dragover = ref(false)
  const fileInput = ref<HTMLInputElement | null>(null)

  const fileMeta = ref<FileMeta | null>(null)
  const result = ref(false)
  const busy = ref(false)
  const progress = ref(0)
  const progressMsg = ref('准备中...')
  const errorMsg = ref('')
  const processedReplays = ref<ProcessedReplayItem[]>([])

  const opts = reactive({
    analyze_type: "multi",
    output_dir: '' as string,

    fulltime: false,
    cancel: true,
    buildList: true,
    upgradeList: true,
    UnitList: true,
    terranFly: false,
    workerNumber: true,
    exportXlsx: true,
    exportTxt: true,
    exportSummary: true
    // tz: 'local',
    // lang: 'zh'
  })

function formatDuration(sec: number) {
  const m = Math.floor(sec / 60)
  const s = sec % 60
  return `${m}:${s.toString().padStart(2, '0')}`
}

async function openTotalExportDir() {
  if(opts.output_dir) {
     await openPathSafe(opts.output_dir) // 打开总的数据目录，或者你可以记录最后一次的 output
  }
}

async function openPathSafe(path: string) {
  try {
    await openPath(path)
  } catch (e) {
    console.error("Open path failed", e)
  }
}


  async function pickFile() {
    const selected = await open({
      directory: true,
      multiple: true,
    })

    if (!selected) return

    console.log('selected:', selected)
    const path = Array.isArray(selected) ? selected[0] : selected
    const name = path.split(/[/\\]/).pop() || '回放文件'
    console.log('name:', name)
    console.log('path:', path)

    await updateFileMetaFromPath(path, name)
    result.value = false
    errorMsg.value = ''
  }


  async function updateFileMetaFromPath(path: string, nameFromCaller?: string) {
    try {
      const info: FileInfo = await stat(path)  // 来自 @tauri-apps/plugin-fs

      const fallbackName = path.split(/[/\\]/).pop() || '回放文件'
      const name = nameFromCaller ?? fallbackName
      const replayCount = await invoke<number>('get_replay_meta', { path })
      const dir = await dirname(path)
      console.log("replayCount:",replayCount)
      const mtime = formatFileTime(info.mtime)
      const ctime = formatFileTime(info.birthtime ?? info.mtime ?? null)

      fileMeta.value = {
        name,
        count: replayCount, 
        ctime: ctime || mtime,
        mtime,
        path,
      }
    } catch (err) {
      console.error('stat failed', err)
      const now = new Date().toLocaleString()
      const fallbackName = path.split(/[/\\]/).pop() || '回放文件'
      const name = nameFromCaller ?? fallbackName

      // 出错时给一个兜底信息
      fileMeta.value = {
        name,
        count:0, 
        ctime: now,
        mtime: now,
        path,
      }
    }

    result.value = false
    errorMsg.value = ''
  }

  function formatFileTime(d: Date | null): string {
    if (!d) return ''
    return d.toLocaleString()
  }

  function onFile(e: Event) {
    const f = (e.target as HTMLInputElement).files?.[0]
    if (!f) return
    attachFile(f)
  }

  function onDrop(e: DragEvent) {
    dragover.value = false
    const f = e.dataTransfer?.files?.[0]
    if (!f) return
    attachFile(f)
  }

  async function attachFile(f: File) {
    const anyFile = f as any
    const path: string | undefined = anyFile.path

    if (path) {
      await updateFileMetaFromPath(path, f.name)
    } else {
      const time = new Date(f.lastModified).toLocaleString()
      fileMeta.value = {
        name: f.name,
        count: 1,                  // 无法遍历目录，只能认为至少有当前这一个
        ctime: time,
        mtime: time,
        path: '本地选择的回放',
      }
      result.value = false
      errorMsg.value = ''
    }
  }

  function clearFile() {
    fileMeta.value = null
    result.value = false
  }

  function resetOptions() {
    opts.analyze_type = "multi"
    opts.fulltime= false
    opts.cancel= true
    opts.buildList= true
    opts.upgradeList= true
    opts.UnitList= true
    opts.terranFly= false
    opts.workerNumber= true
    opts.exportXlsx= true
    opts.exportTxt= true
    opts.exportSummary= true
    // opts.tz = 'local'
    // opts.lang = 'zh'
  }
  async function openExportDir() {
    const baseDir = await appDataDir(); 
    const exportDir = await join(baseDir, 'replays');

    console.log("exportDir:", exportDir)
    try {
      await openPath(exportDir);
      console.log("已打开导出目录:", exportDir)
    } catch (e) {
      console.error("打开目录失败:", e)
    }
    console.log('open export dir') 
}

  async function openSummary() {
    const baseDir = await appDataDir(); 
    const exportDir = await join(baseDir, 'summary');

    console.log("exportDir:", exportDir)
    try {
      await openPath(exportDir);
      console.log("已打开导出目录:", exportDir)
    } catch (e) {
      console.error("打开目录失败:", e)
    }
    console.log('open export dir') 
}


  let unlisten: UnlistenFn | null = null
  let unlistenProgress: UnlistenFn | null = null

  let unlistenFileDrop: UnlistenFn | null = null

  onMounted(async () => {
    opts.output_dir = await appDataDir()
  })

  // onBeforeUnmount(() => {
  //   if(unlistenProgress) unlistenProgress()
  // })

  onMounted(async () => {
    // 监听 Rust 侧发来的进度事件
    // unlisten = await listen<{ percent: number }>('analyze_progress', (e) => {
    //   progress.value = Math.max(progress.value, Math.min(99, e.payload.percent ?? 0))
    // })

    opts.output_dir = await appDataDir() 

    unlistenFileDrop = await listen<string[]>('tauri://drag-drop',  async  (event) => {

      const paths = event.payload.paths
      if (!paths || !paths.length) return
      const path = paths[0]
      const name = path.split(/[/\\]/).pop() || '回放文件'
      await updateFileMetaFromPath(path, name)
      result.value = false
      errorMsg.value = ''
    })


  })

  onBeforeUnmount(() => { 
    unlisten?.(); unlisten = null
    unlistenFileDrop?.(); unlistenFileDrop = null
  })

  async function startAnalyze() {
    if (!fileMeta.value) return

    busy.value = true
    progress.value = 0
    progressMsg.value = '启动分析引擎...'
    processedReplays.value = [] 
    errorMsg.value = ''

      try {
        // 进度条
      if(unlistenProgress) unlistenProgress() // 防止重复监听
        
      unlistenProgress = await listen<ProgressPayload>('analyze_progress', (event) => {
        // 接收 Rust/Python 发来的进度
        const pl = event.payload
        
        // 更新 UI
        progress.value = pl.percentage
        progressMsg.value = pl.message

        if (pl.replayInfo && pl.replayName) {
        // 将其添加到列表顶部 (unshift) 或者 底部 (push)
        // 这里用 unshift 让最新的显示在最上面
          processedReplays.value.unshift({
            fileName: pl.replayName,
            data: pl.replayInfo
          })
        }
      });

    // invoke Rust command
      console.log("Starting analysis task...", opts)

      console.log("fileMeta.value.path:",fileMeta.value.path)
      console.log("opts:",opts)
      await invoke<AnalyzeResult>('analyze_replay', {
        path: fileMeta.value.path,
        options: {...opts }
      })
      progress.value = 100
      progressMsg.value = "分析完成"
      result.value = true
    } catch (e: any) {
      console.error(e)
      errorMsg.value = e?.message || '分析失败，请稍后重试'
    } finally {
      if (unlistenProgress) {
        unlistenProgress() 
        unlistenProgress = null
      }
      busy.value = false
    }
  }




  </script>

  <style scoped>


  .page { display: flex; flex-direction: column; height: 100%; background: var(--bg); color: var(--text); }
  /* .toolbar {
    display: flex; align-items: center; justify-content: space-between; padding: 14px 20px; border-bottom: 1px solid var(--border); background: #0e1012; position: sticky; top: 0; z-index: 2; 
    } */
  .title { font-size: 18px; margin: 4px 4px; font-weight: 700; }
  .toolbar .right { display: flex; align-items: center; gap: 10px; }

  .search-box { display: flex; align-items: center; gap: 8px; background: var(--panel); border: 1px solid var(--border); padding: 8px 10px; border-radius: 12px; width: 320px; }
  .search-box .icon { width: 18px; height: 18px; color: var(--muted); }
  .search-box input { background: transparent; border: none; outline: none; color: var(--text); width: 100%; }

  .content {
    margin-top: 50px;
    display: grid;
    grid-template-columns: 420px 1fr;
    gap: 16px;
    padding: 16px 20px; 
    height: calc(100vh - 50px);
    box-sizing: border-box;
    overflow: hidden;
    }
  .left-pane, .right-pane { height: 100%; overflow: auto; padding-right: 6px; }

  .dropzone { border: 1.5px dashed var(--border); background: var(--panel-2); border-radius: 16px; padding: 26px; text-align: center; transition: .15s ease; }
  .dropzone.dragover { border-color: var(--primary); background: #101318; box-shadow: 0 0 0 3px rgba(59,130,246,.12) inset; }
  .dz-inner { display: grid; gap: 6px; place-items: center; }
  .dz-icon { width: 44px; height: 44px; color: var(--muted); }
  .dz-title { font-weight: 600; border:#ffffff}
  .dz-title span { color: var(--primary); }
  .dz-sub { color: var(--muted); }
  .hidden { display: none; }
  .link { color: var(--primary); background: transparent; border: none; cursor: pointer; padding: 0; }

  .card { 
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 14px;
  margin: 12px 0px; }
  .card-header { display: flex; align-items: center; justify-content: space-between; gap: 10px; margin-bottom: 8px; }
  .card-title { font-weight: 700; }
  .card-sub { color: var(--muted); font-size: 12px; }
  .file-name { font-weight: 600; max-width: calc(100% - 80px); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .meta { list-style: none; padding: 0; margin: 0; display: grid; gap: 6px; }
  .meta li { display: flex; justify-content: space-between; border-bottom: 1px dashed var(--border); padding: 6px 0; }
  .meta li:last-child { border-bottom: none; }
  .truncate { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

  .options { display: grid; grid-template-columns: 1fr; gap: 8px; margin: 8px 0 10px; }
  .opt { display: flex; align-items: center; gap: 8px; user-select: none; }
  .row { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin-top: 6px; }
  .col { display: flex; flex-direction: column; gap: 6px; }
  .label { color: var(--muted); font-size: 12px; }
  .select { background: #0f1317; border: 1px solid var(--border); color: var(--text); padding: 8px 10px; border-radius: 10px; }

  .actions { display: flex; gap: 10px; margin-top: 12px; }
  .btn { border: 1px solid var(--border); background: #0f1317; color: var(--text); padding: 8px 12px; border-radius: 10px; cursor: pointer; transition: .15s ease; }
  .btn:hover { filter: brightness(1.1); }
  .btn:disabled { opacity: .5; cursor: not-allowed; }
  .btn.primary { background: var(--primary); border-color: transparent; color: white; }
  .btn.success { background: var(--success); border-color: transparent; color: #0b1b10; }
  .btn.ghost { background: transparent; }
  .btn.tiny { padding: 6px 10px; font-size: 12px; border-radius: 8px; }

  .progress { height: 8px; border-radius: 999px; background: #0f1317; border: 1px solid var(--border); overflow: hidden; margin-top: 10px; }
  .progress .bar { height: 100%; background: linear-gradient(90deg, var(--primary), #67a3ff); width: 0%; transition: width .2s ease; }

  .hint { color: #fca5a5; margin-top: 8px; }

  .placeholder { display: grid; place-items: center; height: 100%; gap: 8px; }
  .muted { color: var(--muted); }

  .result { display: grid; gap: 12px; }
  .overview-grid { display: grid; grid-template-columns: repeat(3, minmax(0,1fr)); gap: 10px; }
  .kv { background: #111418; border: 1px solid var(--border); border-radius: 12px; padding: 12px; display: flex; flex-direction: column; gap: 6px; }
  .kv span { color: var(--muted); font-size: 12px; }
  .kv b { font-size: 16px; }
  .kv b.win { color: var(--success); }
  b.win { color: var(--success); }
  b.loss { color: #f87171; }
  .bo-list { margin: 0; padding-left: 18px; display: grid; gap: 6px; }
  .bo-list .time { color: var(--muted); margin-right: 10px; }

  .export-row { display: flex; align-items: center; gap: 10px; }

  /* 历史抽屉 */
  .history { position: fixed; right: 0; top: 60px; bottom: 0; width: 420px; background: #0e1012; border-left: 1px solid var(--border); padding: 14px; overflow: auto; }
  .history-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; }
  .history-list { display: grid; gap: 10px; }
  .history-card { background: var(--panel); border: 1px solid var(--border); border-radius: 14px; padding: 12px; }
  .history-card .top { display: flex; justify-content: space-between; gap: 12px; }
  .history-card .title { font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .history-card .meta { color: var(--muted); font-size: 12px; }
  .history-card .bottom { margin-top: 10px; display: flex; gap: 10px; }

  /* 过渡 */
  .slide-enter-active, .slide-leave-active { transition: transform .18s ease, opacity .18s ease; }
  .slide-enter-from, .slide-leave-to { transform: translateX(20px); opacity: 0; }

  /* 滚动条 */
  .left-pane::-webkit-scrollbar, .right-pane::-webkit-scrollbar, .history::-webkit-scrollbar { width: 10px; height: 10px; }
  .left-pane::-webkit-scrollbar-thumb, .right-pane::-webkit-scrollbar-thumb, .history::-webkit-scrollbar-thumb { background: #23272e; border-radius: 8px; }
  
  .rep-index{
    font-weight: 700;
    margin: 0px;
    padding: 0px;
    font-size: 16px;
  }

  .rep-map{
    font-weight: 500;
    margin: 0px 5px;
    padding: 0px;
    font-size: 16px;
  }

  .rep-time { 
    color: var(--muted); 
    margin: 0px 5px; 
    font-size: 14px;
  }

  .player-title{
      display: flex; 
      align-items: center; 
      justify-content: space-between;
  }

  .p-name{
    font-weight: 700;
    margin: 0px;
    padding: 0px;
    font-size: 17px;
  }

  .p-race{
    color: var(--muted); 
    font-size: 13px;
    margin: 0px 5px;
  }

  </style>
