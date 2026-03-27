const { chromium } = require('playwright');

(async () => {
    const browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    
    console.log('Navigating to graph page...');
    await page.goto('http://localhost:8080/graph', { waitUntil: 'networkidle', timeout: 60000 });
    
    console.log('\n--- Simulating full filter pipeline ---');
    const result = await page.evaluate(async () => {
        try {
            const response = await fetch('/api/graph/data');
            const data = await response.json();
            
            if (!data.success || !data.data) {
                return { error: 'API returned no data' };
            }
            
            const fullData = data.data;
            
            const docTypes = ['document', 'doc_section'];
            const funcTypes = ['function', 'class', 'struct'];
            
            // Simulate filterTestElements (remove test nodes)
            const isTestElement = (node) => {
                const qn = node.id.toLowerCase();
                const fp = node.file_path ? node.file_path.toLowerCase() : '';
                return qn.includes('test_') || qn.includes('_test.') || qn.endsWith('_test') 
                    || fp.includes('_test.') || fp.includes('/test/') || fp.includes('\\test\\')
                    || fp.includes('benchmark');
            };
            
            const nodeIds = new Set();
            fullData.nodes.forEach(n => { if (!isTestElement(n)) nodeIds.add(n.id); });
            const filteredEdges = fullData.edges.filter(e => nodeIds.has(e.source) && nodeIds.has(e.target));
            const filteredNodes = fullData.nodes.filter(n => nodeIds.has(n.id));
            
            console.log('After filterTestElements:', filteredNodes.length, 'nodes', filteredEdges.length, 'edges');
            
            // Apply 'document' filter (simulating current buggy logic)
            const docNodeIds = new Set();
            filteredNodes.forEach(n => { if (docTypes.includes(n.element_type)) docNodeIds.add(n.id); });
            const docFilteredEdges = [];
            const docEdgeNodeIds = new Set();
            filteredEdges.forEach(e => { 
                if (docNodeIds.has(e.target)) { 
                    docFilteredEdges.push(e); 
                    docEdgeNodeIds.add(e.source); 
                    docEdgeNodeIds.add(e.target); 
                } 
            });
            const docFilteredNodes = filteredNodes.filter(n => docEdgeNodeIds.has(n.id));
            
            console.log('Document filter result:', docFilteredNodes.length, 'nodes', docFilteredEdges.length, 'edges');
            
            // Apply 'function' filter
            const funcNodeIds = new Set();
            filteredNodes.forEach(n => { if (funcTypes.includes(n.element_type)) funcNodeIds.add(n.id); });
            const funcFilteredEdges = [];
            const funcEdgeNodeIds = new Set();
            filteredEdges.forEach(e => { 
                if (funcNodeIds.has(e.source) || funcNodeIds.has(e.target)) { 
                    funcFilteredEdges.push(e); 
                    funcEdgeNodeIds.add(e.source); 
                    funcEdgeNodeIds.add(e.target); 
                } 
            });
            const funcFilteredNodes = filteredNodes.filter(n => funcEdgeNodeIds.has(n.id));
            
            console.log('Function filter result:', funcFilteredNodes.length, 'nodes', funcFilteredEdges.length, 'edges');
            
            // Now check what applyLimitAndOrphan would do
            const applyLimitAndOrphan = (data) => {
                if (data.nodes.length <= 500 && data.edges.length <= 1000) {
                    return data;
                }
                const nodeConnectCount = {};
                data.edges.forEach(e => { 
                    nodeConnectCount[e.source] = (nodeConnectCount[e.source] || 0) + 1; 
                    nodeConnectCount[e.target] = (nodeConnectCount[e.target] || 0) + 1; 
                });
                const sortedNodes = [...data.nodes].sort((a, b) => 
                    (nodeConnectCount[b.id] || 0) - (nodeConnectCount[a.id] || 0)
                );
                const topNodeIds = new Set(sortedNodes.slice(0, 500).map(n => n.id));
                const filteredNodes = data.nodes.filter(n => topNodeIds.has(n.id));
                const filteredEdges = data.edges.filter(e => topNodeIds.has(e.source) && topNodeIds.has(e.target)).slice(0, 1000);
                return { nodes: filteredNodes, edges: filteredEdges };
            };
            
            const docLimited = applyLimitAndOrphan({ nodes: docFilteredNodes, edges: docFilteredEdges });
            const funcLimited = applyLimitAndOrphan({ nodes: funcFilteredNodes, edges: funcFilteredEdges });
            
            console.log('Document filter after limit:', docLimited.nodes.length, 'nodes', docLimited.edges.length, 'edges');
            console.log('Function filter after limit:', funcLimited.nodes.length, 'nodes', funcLimited.edges.length, 'edges');
            
            // Check type distribution in filtered results
            const docTypeCounts = {};
            docLimited.nodes.forEach(n => { docTypeCounts[n.element_type] = (docTypeCounts[n.element_type] || 0) + 1; });
            const funcTypeCounts = {};
            funcLimited.nodes.forEach(n => { funcTypeCounts[n.element_type] = (funcTypeCounts[n.element_type] || 0) + 1; });
            
            return {
                docFilterAfterLimit: { nodes: docLimited.nodes.length, edges: docLimited.edges.length, typeCounts: docTypeCounts },
                funcFilterAfterLimit: { nodes: funcLimited.nodes.length, edges: funcLimited.edges.length, typeCounts: funcTypeCounts }
            };
        } catch (e) {
            return { error: e.message || String(e), stack: e.stack };
        }
    });
    
    console.log('Result:', JSON.stringify(result, null, 2));
    
    await browser.close();
})();
