import React, { useState, useRef, useEffect } from "react";
import Recorder from "./recorder";
import { invoke } from '@tauri-apps/api/tauri';

const AudioRecorder = () => {
    const [isRecording, setIsRecording] = useState(false);
    const recorder = useRef(new Recorder());

    useEffect(() => {
        const initRecorder = async () => {
            await recorder.current.init();
        };
        initRecorder();

        return () => {
            recorder.current.clearRecording();
        };
    }, []);

    const handleStartRecording = () => {
        recorder.current.startRecording();
        setIsRecording(true);
    };

    const sendBlobToRust = async (recordedBlog: Blob | null) => {
        if (!recordedBlog) return;
      
        console.log("Sending blob to rust");
        console.log(recordedBlog);
        await invoke("send_conversation", [{ blob: base64data }])
      }

    const handleStopRecording = async () => {
        setIsRecording(false);

        recorder.current.stopRecording();

        await new Promise<void>((resolve) => {
            const checkBlob = () => {
                const recordedBlob = recorder.current.getRecordedBlob();
                if (recordedBlob) {
                    console.log(recordedBlob);
                    sendBlobToRust(recordedBlob);
                    resolve();
                } else {
                    setTimeout(checkBlob, 100);
                }
            };
            checkBlob();
        });
    };

    const handleDownloadRecording = () => {
        const recordedBlob = recorder.current.getRecordedBlob();
        if (recordedBlob) {
            const url = URL.createObjectURL(recordedBlob);
            const link = document.createElement("a");
            link.href = url;
            link.download = "recording.webm";
            link.click();
            URL.revokeObjectURL(url);
        }
    };

    return (
        <div>
            {isRecording ? (
                <button onClick={handleStopRecording}>Stop Recording</button>
            ) : (
                <button onClick={handleStartRecording}>Start Recording</button>
            )}
            <button onClick={handleDownloadRecording} disabled={isRecording}>
                Download Recording
            </button>
        </div>
    );
};

export default AudioRecorder;
