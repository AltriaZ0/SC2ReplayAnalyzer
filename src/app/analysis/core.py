from .putin import *
from .Analysis import *

# 分析功能的入口，选择处理分支

def analyse_core(json_data):
    """分析核心函数，根据命令行参数调用相应的分析功能"""
    output_dir = Path(json_data["options"]["output_dir"]) 
    if json_data["options"]["analyze_type"] == "alone":
        rep_path = json_data["path"]
        return AloneRep(rep_path, output_dir, json_data["options"], None)
    elif json_data["options"]["analyze_type"] == "multi":
        rep_path = json_data["path"]
        return MultiRep(rep_path, output_dir, json_data["options"], None)

        
def analyse_core_cmd(args):
    if args.cmd == "alone":
        rep_path = "F:\Code\SC2RepAnalysis\\rep\\test.SC2Replay"
        AloneRep(rep_path, Path(rep_path).parent/"SC2ReplayAnalyzer", None)
    elif args.cmd == "multi":
        rep_dir = "F:\Code\SC2RepAnalysis\\rep"
        MultiRep(rep_dir, Path(rep_dir).parent/"SC2ReplayAnalyzer", None)
