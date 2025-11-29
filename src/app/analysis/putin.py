from .Analysis import ReplayAnalyzer
from app.constants import SC2_SUFFIX
from app.libsK import *
from typing import Callable, Iterable, Optional, Dict, Any, List
import logging

LOG = logging.getLogger("app.analysis")

def _is_sc2_replay(path: Path) -> bool:
    # 兼容原逻辑（末尾 9 个字符）+ 更稳妥的后缀判断
    return path.name.endswith("SC2Replay") or path.suffix == SC2_SUFFIX

ProgressCb = Optional[Callable[[str], None]]  # 例如向 Tauri emit 文本

def _emit(progress: ProgressCb, msg: str) -> None:
    if progress:
        try:
            progress(msg)
        except Exception:
            LOG.exception("progress callback failed")

def AloneRep(
    replay_path:str | Path,
    output_dir: str | Path, 
    options: dict[str, Any] | None = None,
    progress: ProgressCb = None,
):
    replay_path = Path(replay_path)
    output_dir = Path(output_dir) / "replays"

    if not replay_path.exists() or not replay_path.is_file():
        raise FileNotFoundError(f"未找到replay文件: {replay_path}")
    if not _is_sc2_replay(replay_path):
        raise ValueError(f"类型无效，请选择 *.SC2Replay 文件: {replay_path}")

    # base_dir = replay_path.parent
    name = replay_path.name
    LOG.info("选取的 Rep 文件路径: %s", replay_path)
    LOG.info("将在此目录创建分析输出: %s\SC2ReplayAnalyzer\%s", output_dir, name)
    _emit(progress, f"开始解析: {name}")
    opts = options or {}

    try:
        analyzer = ReplayAnalyzer(str(replay_path), output_dir, options=opts)
        result = analyzer.analyze() 
    except Exception as e:
        LOG.exception("解析 replay 失败: %s", replay_path)
        raise e
    

    final_result = {
        "type" : "result",
        "payload" : result
    }
    msg = f"=w= ~\ntxt和excel文件已经生成，位于: {output_dir}"
    LOG.info(msg)
    _emit(progress, msg)
    return final_result

def MultiRep(replay_dir:str | Path, output_dir: str | Path, options: dict[str, Any] | None = None, progress: ProgressCb = None) -> None:
    replay_dir = Path(replay_dir)
    summary_dir = output_dir / "summary"
    summary_dir.mkdir(parents=True, exist_ok=True)

    output_dir = Path(output_dir)  / "replays"
    LOG.info("replay_dir:%s",replay_dir)
    opts = options or {}
    S=[]

    repSum=0
    for r,d,f in os.walk(replay_dir):
        for file in f:
            if file[-9:]=="SC2Replay":
                repSum+=1

    repCount = 0
    for r,d,f in os.walk(replay_dir):
        length=len(f)
        for j in f:
            if j[-9:]=='SC2Replay':
                T=ReplayAnalyzer("{}/{}".format(r,j),output_dir,options=opts) 
                repInfo = T.analyze()
                repCount+=1
                progress_msg = {
                    "type": "progress",  # 标记为进度消息
                    "payload": {
                        "percentage": (repCount / repSum) * 100,
                        "message": f"正在解析第 {repCount} 个录像 )...",
                        "replayName": j,
                        "replayPath": "{}/{}".format(r,j),
                        "replayInfo": repInfo
                    }
                }
                print(json.dumps(progress_msg))
                sys.stdout.flush() 
                result = T.PdS
                if T:
                    for i in result:
                        S.append(i)

    dirname=os.path.basename(replay_dir)
    if options is None or options["exportSummary"] :
        S_to_excel = pd.DataFrame(S,columns=['ID','地图','种族','种族对抗',"比赛时间","比赛胜负",'升级顺序','建筑顺序','单位顺序',"rep地址"])
        S_to_excel.to_excel("{}/所有对局的建造列表({}目录下).xlsx".format(summary_dir,dirname),index=False)

    LOG.info("txt和excel文件已经生成，位于{}".format(output_dir))
    os.startfile("{}".format(output_dir))
