
function mxCssLink(href)
{
	var s = document.createElement('link');
	s.setAttribute('rel', 'stylesheet');
	s.setAttribute('href', href);
	
	var t = document.getElementsByTagName('link')[0];
	if (t != null)
	{
		t.parentNode.insertBefore(s, t);
	}
};

function setCellAttribute(cell, name, value) {
	//cell.value = new NamedNodeMap();
	cell.setAttribute(name, value);
}

function loadScadaModel(editor, xmlStr) {
	const node = mxUtils.parseXml(xmlStr).documentElement;
	if (!!node) {
		let dec = new mxCodec(node.ownerDocument);
	
		if (node.nodeName == 'mxGraphModel')
		{
			editor.graph.model.beginUpdate();
			try
			{
				editor.graph.model.clear();
				editor.graph.view.scale = 1;
				editor.readGraphState(node);
				editor.updateGraphComponents();
				dec.decode(node, editor.graph.getModel());
			}
			finally
			{
				editor.graph.model.endUpdate();
			}
	
			editor.fireEvent(new mxEventObject('resetGraphView'));
		}	
	}
}

function getCell0(editor) {
	return editor.graph.getModel().getCell("0");
}

function getPrettyXml(element) {
	return mxUtils.getPrettyXml(element);
}

// двигает модель в (0,0) угол
function clipedModelBox(modelStr) {
	let node = mxUtils.parseXml(modelStr).documentElement;
	if (!!node && node.nodeName === 'mxGraphModel') {
		let container = document.createElement("div");
		let graph2 = new mxGraph(container);
		let codec = new mxCodec(node);

		graph2.model.beginUpdate();
		try
		{
			graph2.model.clear();
			codec.decode(node, graph2.getModel());
		}
		finally
		{
			graph2.model.endUpdate();
		}

		graph2.model.beginUpdate();
		try {
			let cells = graph2.model.cells;
			let widgetCells = Object.entries(cells).map(( [k, v] ) => v);

			let box = graph2.getBoundingBox(widgetCells);
			let x = box.x;
			let y = box.y;
			widgetCells.forEach(c => {
				let geom = c.getGeometry();
				if (!!geom) {
					geom.x -= x;
					geom.y -= y;
				}
			});	
		}
		finally {
			graph2.model.endUpdate();
		}
		
		let encoder = new mxCodec();
		let result = encoder.encode(graph2.model);
		let res =  mxUtils.getXml(result, '');
		container.remove();
		return res;
	}

	return modelStr;
}

function setWidgetModel(editor, cellP, modelStr) {
	let node = mxUtils.parseXml(modelStr).documentElement;
	if (!!node && node.nodeName === 'mxGraphModel') {
		// console.log("setWidgetModel", cellP, node);

		let container = document.createElement("div");
		let graph2 = new mxGraph(container);
		let codec = new mxCodec(node);

		let widgetCells = [];
		graph2.model.beginUpdate();
		try
		{
			graph2.model.clear();
			// graph2.view.scale = 0.5;
			codec.decode(node, graph2.getModel());
		}
		finally
		{
			graph2.model.endUpdate();
		}

		graph2.model.beginUpdate();
		try {
			let cells = graph2.model.cells;
			widgetCells = Object.entries(cells).map(( [k, v] ) => v);

			let box = graph2.getBoundingBox(widgetCells);
			let pgeom = cellP.getGeometry();
			pgeom.width = box.width;
			pgeom.height = box.height;

			let x = box.x;
			let y = box.y;
			let idp = cellP.getId();
			widgetCells.forEach(c => {
				let geom = c.getGeometry();
				if (!!geom) {
					geom.x -= x;
					geom.y -= y;
				}
				let newId =idp + '#' + c.getId();
				c.setId(newId);
			});	
		}
		finally {
			graph2.model.endUpdate();
		}

		// merge into cellP
		editor.graph.model.beginUpdate();
		try {
			let cells = graph2.model.cells;
			editor.graph.model.mergeChildren(cells["1"], cellP, false);
		}
		finally {
			editor.graph.model.endUpdate();
			container.remove();
		}

		editor.fireEvent(new mxEventObject('resetGraphView'));
	}
}

function getGraphSvg(editor) {
		/*
		Graph.prototype.getSvg = function(
		background,  --optional collor
		scale, 	--optional
		border,  --optional
		nocrop,  --optional
		crisp, --optional
		ignoreSelection, --optional bool
		showText, --optional bool
		imgExport,  --null
		linkTarget, 
		hasShadow, 
		incExtFonts,
		theme, 
		exportType, 
		cells, 
		noCssClass, 
		disableLinks --true
		)
		*/

		let svg = editor.graph.getSvg();
		return mxUtils.getXml(svg, '');
}

/**
 * Sample plugin.
 */
Draw.loadPlugin(async function(ui) {
	const {initSync, renderCell, recreateModelMeta, openDialog, SchemaOptions, initSchemaRender, initCellRender} = await import('./lib/pkg/scada_lib.js');

	async function initWasm() {
		await fetch('plugins/scada/lib/pkg/scada_lib_bg.wasm')
			.then(r => r.arrayBuffer())
			.then(o => {
				initSync(o);
			});				
	}
	// ============= CSS =====================
	mxCssLink("plugins/scada/css/styles.css");
	// mxCssLink("plugins/scada/css/iconfont/material-icons.css");

	// ============= windows ==================
	let diagramDataWindow = null;
	let cellDataWindow = null;

	//--------------------------------------------------------
	let schemaDiv = document.createElement('div');
	schemaDiv.setAttribute("id", "container");
	schemaDiv.style.background = Editor.isDarkMode() ? Editor.darkColor : '#ffffff';
	schemaDiv.style.border = '1px solid gray';
	schemaDiv.style.opacity = '0.8';
	schemaDiv.style.padding = '10px';
	schemaDiv.style.paddingTop = '0px';
	schemaDiv.style.width = '20%';

	let graph = ui.editor.graph;

	if (!ui.editor.isChromelessView())
	{
		schemaDiv.style.boxSizing = 'border-box';
		schemaDiv.style.minHeight = '100%';
		schemaDiv.style.width = '100%';

		let iiw = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
		
		// main window
		diagramDataWindow = new mxWindow('DFlow diagram data', schemaDiv, iiw - 320, 60, 300, 500, true, true);
		diagramDataWindow.destroyOnClose = false;
		diagramDataWindow.setMaximizable(true);
		diagramDataWindow.setResizable(true);
		diagramDataWindow.setScrollable(true);
		diagramDataWindow.setClosable(true);
		diagramDataWindow.contentWrapper.style.overflowY = 'scroll';
	}
	else
	{
		schemaDiv.style.position = 'absolute';
		schemaDiv.style.minWidth = '200px';
		schemaDiv.style.top = '40px';
		schemaDiv.style.right = '20px';

		document.body.appendChild(schemaDiv);
	}
	
	// Highlights current cell
	const highlight = new mxCellHighlight(graph, '#00ff00', 2);
	// const ignored = ['label', 'tooltip', 'placeholders'];

	// register_conteiner(ui.editor, div);	// for wasm app
	
	// init wasm application
	// const app = new AppApi(ui.editor, div);

	function writeConsole(evt)
	{
		let result = graph.getDataForCells(graph.getSelectionCells());

		if (mxEvent.isShiftDown(evt))
		{
			console.log(JSON.stringify(result, null, '  '));
		}
		else
		{
			console.log(result);
		}
	};

	// let prevcell = undefined;
	/**
	 * Updates the properties panel
	 */
	let isRendered = false;
	function cellClicked(cell, modelChanged)
	{
		// Gets the selection cell
		if (cell == null)
		{
			highlight.highlight(null);
			if (!isRendered) {
				// console.log("CALL renderSchema");
				// renderSchema(mxUtils, ui.editor, schemaDiv, getAppOptions());
				isRendered = true;
			}
		}
		else
		{
			if (ui.editor.isChromelessView())
			{
				highlight.highlight(graph.view.getState(cell));
			}
	
			// console.log("compare prev", prevcell===cell);

			if (modelChanged) {
				console.log("model changed", cell);
				// app.cell_updated(cell);
			} else {
				//let doc = mxUtils.parseXml("<d-flow><som-data p='test' as='data'/></d-flow>").documentElement;
				//cell.setValue(doc);

				//console.log("selection changed", cell.value);
				// app.cell_clicked(cell);

				// renderCell(div, cell);
				// prevcell = cell;
			}

			// let attrs = (cell.value != null) ? cell.value.attributes : null;
			// if (attrs != null)
			// {
			// 	let label = Graph.sanitizeHtml(graph.getLabel(cell));
				
			// 	if (label != null && label.length > 0)
			// 	{
			// 		div.innerHTML = '<h1>' + label + '</h1>';
			// 	}
			// 	else
			// 	{
			// 		div.innerText = '';
			// 	}
				
			// 	for (let i = 0; i < attrs.length; i++)
			// 	{
			// 		if (mxUtils.indexOf(ignored, attrs[i].nodeName) < 0 &&
			// 			attrs[i].nodeValue.length > 0)
			// 		{
			// 			// TODO: Add click handler on h2 to output data
			// 			let h2 = document.createElement('h2');
			// 			mxUtils.write(h2, attrs[i].nodeName);
			// 			div.appendChild(h2);
			// 			let p = document.createElement('p');
			// 			mxUtils.write(p, attrs[i].nodeValue);
			// 			div.appendChild(p);
			// 		}
			// 	}

			// 	// set_cell(div, cell);
			// }
			// else
			// {
			// 	let label = graph.convertValueToString(cell);
				
			// 	if (label != '')
			// 	{
			// 		div.innerHTML = '<h1>' + Graph.sanitizeHtml(label) + '</h1>';
			// 	}
			// 	else
			// 	{
			// 		div.innerHTML = '<p><i>No data</i></p>';
			// 	}
			// }

			// if (!ui.editor.isChromelessView())
			// {
			// 	let button = document.createElement('button');
			// 	button.setAttribute('title', 'Click or Shift+Click to write data for all selected cells to the browser console');
			// 	button.style['float'] = 'none';
			// 	mxUtils.write(button, 'Write to Console');

			// 	mxEvent.addListener(button, 'click', function(evt)
			// 	{
			// 		writeConsole(evt);
			// 	});

			// 	div.appendChild(button);
			// }
		}
	};

	if (!ui.editor.isChromelessView())
	{
		graph.selectionModel.addListener(mxEvent.CHANGE, function(sender, evt)
		{
			cellClicked(graph.getSelectionCell(), false);
		});
		
		graph.model.addListener(mxEvent.CHANGE, function(sender, evt)
		{
			cellClicked(graph.getSelectionCell(), true);
		});
	}
	else
	{
		graph.click = function(me)
		{
			// Async required to enable hyperlinks in labels
			window.setTimeout(function()
			{
				cellClicked(me.getCell());
			}, 0);
		};
	}

	// ================== SIDEBAR ===================
	// Adds sidebar entries
	let sb = ui.sidebar;
	function addPalette()
	{
		sb.addPalette('dflow', 'DFlow items', false, function(content)
		{
			(function()
			{
				let cotainer = new mxCell('', new mxGeometry(0, 0, 112, 73), 'container=1;collapsible=0;connectable=0;strokeColor=none;');
				cotainer.vertex = true;

				let value = mxUtils.parseXml("<d-flow><widget uuid='00000000-0000-0000-0000-000000000000' group='valves'/></d-flow>").documentElement;
				value.setAttribute('label', cotainer.value || '');
				cotainer.setValue(value);				

				let glyph = new mxCell('', new mxGeometry(0, 0, 112, 73),
					'shape=image;verticalLabelPosition=bottom;labelBackgroundColor=default;verticalAlign=top;aspect=fixed;imageAspect=0;image=data:image/svg+xml,PHN2ZyB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiBzdHlsZT0iYmFja2dyb3VuZC1jb2xvcjogcmdiKDI1NSwgMjU1LCAyNTUpOyIgdmlld0JveD0iLTAuNSAtMC41IDExMiA3MyIgaGVpZ2h0PSI3M3B4IiB3aWR0aD0iMTEycHgiIHZlcnNpb249IjEuMSI+PGRlZnMvPjxyZWN0IHk9IjAiIHg9IjAiIGhlaWdodD0iMTAwJSIgd2lkdGg9IjEwMCUiIGZpbGw9IiNmZmZmZmYiLz48Zz48ZyBkYXRhLWNlbGwtaWQ9IjAiPjxnIGRhdGEtY2VsbC1pZD0iMSI+PGcgZGF0YS1jZWxsLWlkPSJnTGFUMDk1UEJzMVowd2FzcVNmLS0yIj48Zz48cGF0aCBwb2ludGVyLWV2ZW50cz0iYWxsIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSg4MS4zOSwwKXNjYWxlKC0xLDEpdHJhbnNsYXRlKC04MS4zOSwwKSIgc3Ryb2tlLW1pdGVybGltaXQ9IjEwIiBzdHJva2Utd2lkdGg9IjUiIHN0cm9rZT0iIzAwMDAwMCIgZmlsbD0iIzk5OTk5OSIgZD0iTSA1NC43OSAyIEwgMTA4IDM2IEwgNTQuNzkgNzAgWiIvPjwvZz48L2c+PGcgZGF0YS1jZWxsLWlkPSJnTGFUMDk1UEJzMVowd2FzcVNmLS0zIj48Zz48cGF0aCBwb2ludGVyLWV2ZW50cz0iYWxsIiBzdHJva2UtbWl0ZXJsaW1pdD0iMTAiIHN0cm9rZS13aWR0aD0iNSIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjODA4MDgwIiBkPSJNIDEuNTggMiBMIDU0Ljc5IDM2IEwgMS41OCA3MCBaIi8+PC9nPjwvZz48L2c+PC9nPjwvZz48L3N2Zz4=;');
				glyph.vertex = true;
				glyph.setParent(cotainer);
				
				content.appendChild(sb.createVertexTemplateFromCells([cotainer], 100, 40, 'Valve'));
			})();

		});
	}
	addPalette();

	// Handles reload of sidebar after dark mode change
	let init = sb.init;
	sb.init = function()
	{
		init.apply(this, arguments);
		addPalette();
	};


	// ================ MENUS =================
	// Adds menu
	mxResources.parse('createDiagram=New Diagram');
	mxResources.parse('createWidget=New Widget');
	mxResources.parse('openItem=Open...');
	mxResources.parse('dflow=DFlow');
	mxResources.parse('scadaData=DFlow Data');

	
    ui.actions.addAction('createDiagram', function()
    {
		loadScadaModel(ui.editor, '<mxGraphModel dx="1173" dy="736" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="850" pageHeight="1100" math="0" shadow="0"><root><object label="" id="0"><diagram /><mxCell /></object><mxCell id="1" parent="0" /></root></mxGraphModel>')
		recreateModelMeta("diagram");
		diagramDataWindow.setVisible(true);
    });

    ui.actions.addAction('createWidget', function()
    {
		loadScadaModel(ui.editor, '<mxGraphModel dx="1173" dy="736" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="850" pageHeight="1100" math="0" shadow="0"><root><object label="" id="0"><widget object-type=""/><mxCell /></object><mxCell id="1" parent="0" /></root></mxGraphModel>')
		recreateModelMeta("widget");
		diagramDataWindow.setVisible(true);
    });

    ui.actions.addAction('openItem', function()
    {
		ui.showDialog(new DFlowItemsDialog(ui).container, 500, 400, true, false);
    });	

	ui.actions.addAction('dflow', function()
	{
		diagramDataWindow.setVisible(!diagramDataWindow.isVisible());
	});

	ui.actions.addAction('scadaData', function()
	{
		cellDataWindow.setVisible(!cellDataWindow.isVisible());
	});	

	ui.menus.put('dflow', new Menu(function(menu, parent)
	{
		ui.menus.addMenuItems(menu, ['createDiagram', 'createWidget', '-', 'openItem', '-', 'dflow', 'scadaData']);
	}));

    if (ui.menubar != null)
    {
		var menu = ui.menubar.addMenu('DFlow', ui.menus.get('dflow').funct);
		menu.parentNode.insertBefore(menu, menu.previousSibling.previousSibling.previousSibling);
    }

	// -----------------------------------------------------------------
	let divScadaCellData = document.createElement('div');
	divScadaCellData.setAttribute("id", "cell-container");
	divScadaCellData.style.background = Editor.isDarkMode() ? Editor.darkColor : '#ffffff';
	divScadaCellData.style.border = '1px solid gray';
	divScadaCellData.style.opacity = '0.8';
	divScadaCellData.style.width = '20%';

	divScadaCellData.style.boxSizing = 'border-box';
	divScadaCellData.style.minHeight = '100%';
	divScadaCellData.style.width = '100%';

	
	// cell window
	function newCellWindow(div) {
		let iiw = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
		let wnd = new mxWindow('DFlow data', div, iiw - 320, 60, 300, 450, true, true);
		wnd.destroyOnClose = false;
		wnd.setMaximizable(true);
		wnd.setResizable(true);
		wnd.setScrollable(true);
		wnd.setClosable(true);
		wnd.contentWrapper.style.overflowY = 'scroll';
		return wnd;	
	}

	function isScadaCell(cell)
	{
		if (!!cell && !!cell.value && typeof cell.value !== 'string')
		{
			return cell.value.tagName === "d-flow";
		}
		return false;
	};


	/**
	 * Updates the DFlow data panel
	 */
	// let prevCellId = undefined;
	function scadaCellClicked(cell)
	{
		// Gets the selection cell
		if (cell != null && isScadaCell(cell))
		{
			highlight.highlight(graph.view.getState(cell));

			renderCell(cell);
			cellDataWindow.setVisible(true);
		} 
		else {
			highlight.highlight(null);
			if (cellDataWindow != null) {
				cellDataWindow.setVisible(false);
			}
		}

	}	

	if (!ui.editor.isChromelessView())
	{
		graph.selectionModel.addListener(mxEvent.CHANGE, function(sender, evt)
		{
			scadaCellClicked(graph.getSelectionCell());
		});
	}	

	// Adds resources for actions
	mxResources.parse('scadaItem=DFlow item');

	// Adds actions
	ui.actions.addAction('scadaItem', function()
	{
		if (graph.isEnabled() && graph.getSelectionCount() == 1)
		{
			let cell = graph.getSelectionCell();
			if (!isScadaCell(cell)) {
				let value = mxUtils.parseXml("<d-flow><undefiend/></d-flow>").documentElement;
				value.setAttribute('label', cell.value || '');
				cell.setValue(value);
				scadaCellClicked(cell);
			}
		}		
	}, null, null, 'Alt+Shift+W');

	// -----------------------------------------------------------------
	let uiCreatePopupMenu = ui.menus.createPopupMenu;
	ui.menus.createPopupMenu = function(menu, cell, evt)
	{
		uiCreatePopupMenu.apply(this, arguments);

		menu.addSeparator();
		// let cell = graph.getSelectionCell();
		if (!isScadaCell(cell)) {
			this.addMenuItems(menu, ['scadaItem'], null, evt);
		}

		if (isScadaCell(cell) && graph.getSelectionCount() == 1)
		{
			this.addMenuItems(menu, ['scadaData'], null, evt);

			// if (sib != null && sib.length > 0)
			// {
			// 	this.addMenuItems(menu, ['selectChildren', 'selectSubtree'], null, evt);
			// }
			
			// menu.addSeparator();
			
			// if (cell.getAttribute('treeRoot') != '1')
			// {
			// 	this.addMenuItems(menu, ['selectSiblings', 'selectParent'], null, evt);
			// }
		}
	};
	// =======================================

	// ============== WASM ===================
	// init rust wasm
	await initWasm();
	// здесь натройки пдагина
	let getAppOptions = function() {return new SchemaOptions("http://localhost:8091/api/v1"); }		

	initSchemaRender(ui.editor, mxUtils, schemaDiv, getAppOptions());

	cellDataWindow = newCellWindow(divScadaCellData);
	initCellRender(ui.editor, mxUtils, divScadaCellData, getAppOptions());	


	let DFlowItemsDialog = function(editorUi) 
	{
		var div = document.createElement('div');
		// var inner = document.createElement('div');
		
		// // inner.style.width = '600px';
		// inner.style.height = '300px';
		// inner.style.overflow = 'auto';
	
		// var changed = false;
						
		// open schema items dialod
		openDialog(mxUtils, editorUi, editorUi.editor, div, getAppOptions());
		
		// // div.appendChild(inner);
		// changed = false;
		
		// var cancelBtn = mxUtils.button(mxResources.get('cancel'), function()
		// {
		// 	editorUi.hideDialog();
		// });
		
		// cancelBtn.className = 'geBtn';
		
		// var openBtn = mxUtils.button(closeOnly? mxResources.get('close') : mxResources.get('open'), function()
		// {
		// 	if (changed)
		// 	{
		// 		editorUi.hideDialog();
		// 		editorUi.alert(mxResources.get('restartForChangeRequired'));
		// 	}
		// 	else
		// 	{
		// 		editorUi.hideDialog();
		// 	}	
		// });
		
		// openBtn.className = 'geBtn gePrimaryBtn';
	
		// var buttons = document.createElement('div');
		// buttons.style.marginTop = '14px';
		// buttons.style.textAlign = 'right';
	
		
		// if (editorUi.editor.cancelFirst)
		// {
		// 	if (!closeOnly)
		// 	{
		// 		buttons.appendChild(cancelBtn);
		// 	}
	
		// 		buttons.appendChild(openBtn);
		// }
		// else
		// {
		// 	buttons.appendChild(openBtn);
		// 	if (!closeOnly)
		// 	{
		// 		buttons.appendChild(cancelBtn);
		// 	}
		// }
	
		// div.appendChild(buttons);
		this.container = div;
	};	


});