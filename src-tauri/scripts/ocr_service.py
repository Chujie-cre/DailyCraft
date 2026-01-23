#!/usr/bin/env python3
"""
OCR常驻服务
使用RapidOCR进行文字识别，通过stdin/stdout通信
模型只加载一次，大幅降低CPU占用
"""

import sys
import json
import os

# 限制ONNX线程数
os.environ['OMP_NUM_THREADS'] = '1'
os.environ['MKL_NUM_THREADS'] = '1'
os.environ['OPENBLAS_NUM_THREADS'] = '1'
os.environ['VECLIB_MAXIMUM_THREADS'] = '1'
os.environ['NUMEXPR_NUM_THREADS'] = '1'

# 降低进程优先级
try:
    import psutil
    p = psutil.Process()
    p.nice(psutil.IDLE_PRIORITY_CLASS if hasattr(psutil, 'IDLE_PRIORITY_CLASS') else 19)
except:
    pass

def main():
    # 初始化OCR引擎（只加载一次）
    try:
        from rapidocr_onnxruntime import RapidOCR
        ocr = RapidOCR(intra_op_num_threads=1, inter_op_num_threads=1)
        # 发送就绪信号
        print(json.dumps({"status": "ready"}), flush=True)
    except ImportError:
        print(json.dumps({"error": "RapidOCR未安装，请运行: pip install rapidocr_onnxruntime"}), flush=True)
        sys.exit(1)
    except Exception as e:
        print(json.dumps({"error": f"OCR初始化失败: {str(e)}"}), flush=True)
        sys.exit(1)
    
    # 循环处理请求
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        
        try:
            request = json.loads(line)
            image_path = request.get("image_path", "")
            
            if not image_path:
                print(json.dumps({"error": "缺少image_path参数"}), flush=True)
                continue
            
            if not os.path.exists(image_path):
                print(json.dumps({"error": f"图片不存在: {image_path}"}), flush=True)
                continue
            
            # 执行OCR识别
            result, _ = ocr(image_path)
            
            # 提取文本
            texts = []
            if result:
                for item in result:
                    if item and len(item) >= 2:
                        texts.append(item[1])
            
            print(json.dumps({
                "success": True,
                "text": "\n".join(texts)
            }, ensure_ascii=False), flush=True)
            
        except json.JSONDecodeError:
            print(json.dumps({"error": "无效的JSON请求"}), flush=True)
        except Exception as e:
            print(json.dumps({"error": str(e)}), flush=True)

if __name__ == "__main__":
    main()
