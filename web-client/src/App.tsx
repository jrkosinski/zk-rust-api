import { QueryClientProvider } from '@tanstack/react-query';
import { queryClient } from './lib/queryClient';
import { TreeVisualization } from './components/TreeVisualization';

function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <TreeVisualization />
        </QueryClientProvider>
    );
}

export default App;
