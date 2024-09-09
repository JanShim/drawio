// import { initSync, renderCell, renderSchema, SchemaOptions } from './lib/pkg/scada_lib.js';

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

/**
 * Sample plugin.
 */
Draw.loadPlugin(async function(ui) {
	const {initSync, renderCell, renderSchema, SchemaOptions} = await import('./lib/pkg/scada_lib.js');

	async function initWasm() {
		await fetch('plugins/scada/lib/pkg/scada_lib_bg.wasm')
			.then(r => r.arrayBuffer())
			.then(o => {
				initSync(o);
			});				
	}

	let div = document.createElement('div');
	div.setAttribute("id", "container");
	div.style.background = Editor.isDarkMode() ? Editor.darkColor : '#ffffff';
	div.style.border = '1px solid gray';
	div.style.opacity = '0.8';
	div.style.padding = '10px';
	div.style.paddingTop = '0px';
	div.style.width = '20%';

	let graph = ui.editor.graph;

	if (!ui.editor.isChromelessView())
	{
		div.style.boxSizing = 'border-box';
		div.style.minHeight = '100%';
		div.style.width = '100%';

		let iiw = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
		
		// main window
		let dataWindow = new mxWindow('IIoT-Hub diagram data', div, iiw - 320, 60, 300, 500, true, true);
		dataWindow.destroyOnClose = false;
		dataWindow.setMaximizable(true);
		dataWindow.setResizable(true);
		dataWindow.setScrollable(true);
		dataWindow.setClosable(true);
		dataWindow.contentWrapper.style.overflowY = 'scroll';

		// Adds resource for action
		mxResources.parse('iiot=IIoT-Hub');

		// Adds action
		ui.actions.addAction('iiot', function()
		{
			dataWindow.setVisible(!dataWindow.isVisible());
		});
		
		let menu = ui.menus.get('extras');
		let oldFunct = menu.funct;
		
		menu.funct = function(menu, parent)
		{
			oldFunct.apply(this, arguments);
			ui.menus.addMenuItems(menu, ['-', 'iiot'], parent);
		};
	}
	else
	{
		div.style.position = 'absolute';
		div.style.minWidth = '200px';
		div.style.top = '40px';
		div.style.right = '20px';

		document.body.appendChild(div);
	}
	
	// Highlights current cell
	const highlight = new mxCellHighlight(graph, '#00ff00', 8);
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
	function cellClicked(cell, modelChanged)
	{
		// Gets the selection cell
		if (cell == null)
		{
			highlight.highlight(null);
			// app.cell_clicked(null);
			// renderSchema(div, new SchemaOptions("http://zheleschikovav.keenetic.pro:18764/v1/configurator"));
			renderSchema(mxUtils, ui.editor, div, new SchemaOptions("http://localhost:8091/api/v1"));
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
				//let doc = mxUtils.parseXml("<iiot><som-data p='test' as='data'/></iiot>").documentElement;
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
		sb.addPalette('iiot', 'IIoT', false, function(content)
		{
			(function()
			{
				let cell = new mxCell('Valve', new mxGeometry(0, 0, 100, 40),
					'rectangle;whiteSpace=wrap;html=1;align=center;collapsible=0;container=1;recursiveResize=0;');
				cell.vertex = true;

				let value = mxUtils.parseXml("<iiot><widget uuid='00000000-0000-0000-0000-000000000000'/></iiot>").documentElement;
				value.setAttribute('label', cell.value || '');
				cell.setValue(value);



				// let value = null;
				// if (cell.value != null && typeof(cell.value) == 'object')
				// {
				// 	value = cell.value.cloneNode(true);
				// }
				// else
				// {
				// }
				
				// if (attributeValue != null)
				// {
				// 	value.setAttribute(attributeName, attributeValue);
				// }
				// else
				// {
				// 	value.removeAttribute(attributeName);
				// }

				
				content.appendChild(sb.createVertexTemplateFromCells([cell], 100, 40, 'Valve'));
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
	let divScadaCellData = document.createElement('div');
	divScadaCellData.setAttribute("id", "container");
	divScadaCellData.style.background = Editor.isDarkMode() ? Editor.darkColor : '#ffffff';
	divScadaCellData.style.border = '1px solid gray';
	divScadaCellData.style.opacity = '0.8';
	divScadaCellData.style.padding = '10px';
	divScadaCellData.style.paddingTop = '0px';
	divScadaCellData.style.width = '20%';

	divScadaCellData.style.boxSizing = 'border-box';
	divScadaCellData.style.minHeight = '100%';
	divScadaCellData.style.width = '100%';

	let iiw = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
	
	// main window
	let scadaDataWindow = new mxWindow('IIoT-Hub data', divScadaCellData, iiw - 320, 60, 300, 450, true, true);
	scadaDataWindow.destroyOnClose = false;
	scadaDataWindow.setMaximizable(true);
	scadaDataWindow.setResizable(true);
	scadaDataWindow.setScrollable(true);
	scadaDataWindow.setClosable(true);
	scadaDataWindow.contentWrapper.style.overflowY = 'scroll';

	function isScadaCell(cell)
	{
		if (cell != null && cell.value !== null && typeof cell.value !== 'string')
		{
			return cell.value.tagName === "iiot";
		}
		return false;
	};


	/**
	 * Updates the iiot data panel
	 */
	function scadaCellClicked(cell)
	{
		// Gets the selection cell
		if (cell != null && isScadaCell(cell))
		{
			highlight.highlight(graph.view.getState(cell));
			scadaDataWindow.setVisible(true);
			renderCell(divScadaCellData, cell);
		} 
		else {
			highlight.highlight(null);
			scadaDataWindow.setVisible(false);
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
	mxResources.parse('scadaData=IIot-Hub Data');
	mxResources.parse('scadaItem=IIot-Hub item');

	// Adds actions
	ui.actions.addAction('scadaData', function()
	{
		scadaDataWindow.setVisible(!scadaDataWindow.isVisible());

		// if (graph.isEnabled() && graph.getSelectionCount() == 1)
		// {
		// 	let cell = graph.getSelectionCell();
		// 	let sib = graph.getOutgoingEdges(cell);
			
		// 	if (sib != null)
		// 	{
		// 		let tmp = [];
				
		// 		for (let i = 0; i < sib.length; i++)
		// 		{
		// 			tmp.push(graph.model.getTerminal(sib[i], false));
		// 		}
				
		// 		graph.setSelectionCells(tmp);
		// 	}
		// }
	}, null, null, 'Alt+Shift+Q');
	ui.actions.addAction('scadaItem', function()
	{
		if (graph.isEnabled() && graph.getSelectionCount() == 1)
		{
			let cell = graph.getSelectionCell();
			if (!isScadaCell(cell)) {
				let value = mxUtils.parseXml("<iiot></iiot>").documentElement;
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


});