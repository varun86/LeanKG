const { chromium } = require('playwright');

(async () => {
    const browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    
    const errors = [];
    page.on('console', msg => {
        if (msg.type() === 'error') errors.push(msg.text());
    });
    page.on('pageerror', err => errors.push(err.message));
    
    console.log('Testing filter functionality...');
    await page.goto('http://localhost:8080/graph', { waitUntil: 'networkidle', timeout: 60000 });
    await page.waitForTimeout(3000);
    
    // Check initial render
    let canvases = await page.$$('canvas');
    console.log('Initial (All filter) - Canvas count:', canvases.length);
    
    // Click Document filter
    console.log('\nClicking Document filter...');
    await page.click('.filter-btn[data-filter="document"]');
    await page.waitForTimeout(2000);
    canvases = await page.$$('canvas');
    const docContainer = await page.$('#graph-container');
    const docContent = await docContainer.innerHTML();
    console.log('Document filter - Canvas count:', canvases.length, '- Content length:', docContent.length);
    
    // Click Function filter
    console.log('\nClicking Function filter...');
    await page.click('.filter-btn[data-filter="function"]');
    await page.waitForTimeout(2000);
    canvases = await page.$$('canvas');
    const funcContainer = await page.$('#graph-container');
    const funcContent = await funcContainer.innerHTML();
    console.log('Function filter - Canvas count:', canvases.length, '- Content length:', funcContent.length);
    
    // Click All filter
    console.log('\nClicking All filter...');
    await page.click('.filter-btn[data-filter="all"]');
    await page.waitForTimeout(2000);
    canvases = await page.$$('canvas');
    console.log('All filter - Canvas count:', canvases.length);
    
    if (errors.length > 0) {
        console.log('\nErrors:', errors);
    } else {
        console.log('\nNo errors detected');
    }
    
    const allCanvases = canvases.length > 0;
    const noErrors = errors.length === 0;
    
    if (allCanvases && noErrors) {
        console.log('\nFilter test: PASS');
        process.exit(0);
    } else {
        console.log('\nFilter test: FAIL');
        process.exit(1);
    }
    
    await browser.close();
})();
