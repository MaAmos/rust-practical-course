import React, { useState } from 'react';
import { Header } from './components/Header';
import { DropZone } from './components/DropZone';
import { ConverterView } from './components/ConverterView';
import { parseXmindFile } from './utils/xmindParser';
import { convertToMarkdown } from './utils/markdownGenerator';

function App() {
  const [view, setView] = useState('upload'); // 'upload' | 'converting' | 'result'
  const [fileName, setFileName] = useState('');
  const [topicTree, setTopicTree] = useState(null);
  const [mdContent, setMdContent] = useState('');
  const [error, setError] = useState(null);

  const handleFileSelect = async (file) => {
    try {
      setView('converting');
      setFileName(file.name);
      setError(null);

      // Parse
      const tree = await parseXmindFile(file);

      if (!tree) {
        throw new Error("Could not extract topic tree from Xmind file.");
      }

      setTopicTree(tree);

      // Convert
      const markdown = convertToMarkdown(tree);
      setMdContent(markdown);

      setView('result');
    } catch (err) {
      console.error(err);
      setError(err.message || "Failed to process file.");
      setView('upload');
    }
  };

  return (
    <div className="min-h-screen flex flex-col bg-[url('https://grainy-gradients.vercel.app/noise.svg')] bg-[length:200px] bg-fixed">
      {/* Background overlay for cleaner look with noise */}
      <div className="absolute inset-0 bg-slate-900/90 pointer-events-none -z-10"></div>

      <Header />

      <main className="flex-1 container mx-auto flex flex-col h-[calc(100vh-100px)]">
        {view === 'upload' && (
          <div className="flex-1 flex flex-col items-center justify-center fade-in">
            <DropZone onFileSelect={handleFileSelect} />
            {error && (
              <div className="mt-4 p-4 bg-red-500/10 border border-red-500/20 text-red-200 rounded-lg max-w-md text-center">
                {error}
              </div>
            )}
          </div>
        )}

        {view === 'converting' && (
          <div className="flex-1 flex items-center justify-center">
            <div className="animate-pulse flex flex-col items-center gap-4">
              <div className="w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
              <p className="text-xl font-medium text-blue-400">Processing your thoughts...</p>
            </div>
          </div>
        )}

        {view === 'result' && (
          <ConverterView
            fileName={fileName}
            topicTree={topicTree}
            mdContent={mdContent}
          />
        )}
      </main>
    </div>
  );
}

export default App;
