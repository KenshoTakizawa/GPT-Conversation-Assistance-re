type OnError = (error: Error) => void;

interface RecorderOptions {
  onError?: OnError;
}

class Recorder {
  private mediaRecorder: MediaRecorder | null;
  private recordedChunks: BlobPart[];
  private recordedBlob: Blob | null;

  constructor(private options: RecorderOptions = {}) {
    this.mediaRecorder = null;
    this.recordedChunks = [];
    this.recordedBlob = null;
  }

  async init(): Promise<void> {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      this.mediaRecorder = new MediaRecorder(stream);

      this.mediaRecorder.addEventListener("dataavailable", (event) => {
        if (event.data.size > 0) {
          this.recordedChunks.push(event.data);
        }
      });

      this.mediaRecorder.addEventListener("stop", async () => {
        this.recordedBlob = new Blob(this.recordedChunks, {
          type: "audio/webm",
        });
        this.recordedChunks = [];
      });
    } catch (error: any) {
      this.options.onError?.(error);
    }
  }

  startRecording(): void {
    this.mediaRecorder?.start();
  }

  stopRecording(): void {
    if (this.mediaRecorder && this.mediaRecorder.state === "recording") {
      this.mediaRecorder.stop();
    }
  }

  getRecordedBlob(): Blob | null {
    return this.recordedBlob;
  }

  clearRecording(): void {
    this.recordedBlob = null;
  }

  private async saveRecording(blob: Blob, filePath: string): Promise<void> {
    const fileBuffer = await blob.arrayBuffer();
    const data = new Uint8Array(fileBuffer);
    // Tauriのファイル保存処理
  }
}

export default Recorder;
