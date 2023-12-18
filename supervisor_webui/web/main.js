var infos = undefined;

// 初始化函数
function init() {
    document.getElementById("tabs").innerHTML = "<mdui-linear-progress></mdui-linear-progress>";
    mdui.$.ajax({
        url: '/infos',
        method: 'GET',
        dataType: 'json',
        success: function (res) {
            console.log(res);
            infos = res;
            init_generate();
        }
    });
}

// 初始化处理函数
function init_generate() {
    document.getElementById("tabs").innerHTML = "";
    index = 0;
    for (server in infos["Processes"]) {
        index++;
        document.getElementById("tabs").innerHTML +=
            `
<mdui-tab value="tab-${index}">${server}</mdui-tab>
<mdui-tab-panel slot="panel" value="tab-${index}" id="tabs_items_${index}">
</mdui-tab-panel>`;
        if (!infos["Processes"][server]) {
            document.getElementById(`tabs_items_${index}`).innerHTML += "未获取到信息";
        } else {
            infos["Processes"][server].forEach(tabs_item => {
                document.getElementById(`tabs_items_${index}`).innerHTML +=
                    `
<div class="tabs_item">
    <div class="list_item_block item_name" style="width:30%;min-width:80px">
        <span class="item_status ${tabs_item["state"] == 20 ? "status_running" : "status_stopped"}"></span>
        <span>${tabs_item["name"]}</span>
    </div>
    <div class="list_item_block item_info"><p><b>${tabs_item["statename"]}</b><br />${tabs_item["description"]}</p></div>
    <div class="list_item_block item_btn" server="${server}" program="${tabs_item["name"]}" state="${tabs_item["state"]}">
        <mdui-button-icon variant="filled" icon="${tabs_item["state"] == 20 ? "stop" : "play_arrow"}" style="background-color: ${tabs_item["state"] == 20 ? "green" : "brown"}" onclick="action(this)"></mdui-button-icon>
        <mdui-button-icon variant="filled" icon="more_horiz" onclick="more_action(this)"></mdui-button-icon>
    </div>
</div>`
            });
        }
        document.getElementById(`tabs_items_${index}`).innerHTML +=
            `
<div class="server_info">
<div class="server_info_chips">
    <mdui-chip style="background-color: ${infos["Servers"][server]["state"]["statecode"] == 1 ? "greenyellow" : "indianred"};">${infos["Servers"][server]["state"]["statename"]}</mdui-chip>
    <mdui-chip>${infos["Servers"][server]["identification"]}</mdui-chip>
    <mdui-chip>Version: ${infos["Servers"][server]["version"]}</mdui-chip>
    <mdui-chip>API: ${infos["Servers"][server]["api"]}</mdui-chip>
</div>
<mdui-button-icon variant="filled" icon="more_horiz" onclick="more_action_server('${server}')"></mdui-button-icon>
</div>
        `;
    }
    document.querySelector("mdui-tab[value='tab-1']").click();
}

// 启动/停止 按钮
function action(target) {
    let server = target.parentElement.getAttribute("server");
    let program = target.parentElement.getAttribute("program");
    let state = target.parentElement.getAttribute("state");
    PostRequest(`/api/${server}/${state == 20 ? 'stopProcess' : 'startProcess'}`, {
        'a': program,
        'b': true
    }, target, init);
}

var dialog_server = "";
var dialog_program = "";
// 更多按钮
function more_action(target) {
    dialog_server = target.parentElement.getAttribute("server");
    dialog_program = target.parentElement.getAttribute("program");
    // let state = target.parentElement.getAttribute("state");
    document.getElementById("info-tab").value = "item-1";
    document.querySelector("#action-dialog .log_iframe").style.display = "none";
    ObjToTable("state-info", infos["Processes"][dialog_server].filter(item => { return item["name"] == dialog_program })[0])
    let dialog = document.getElementById("action-dialog");
    dialog.headline = dialog_program;
    dialog.open = true;
}

var selected_server = "";
// 服务器更多按钮
function more_action_server(server) {
    selected_server = server;
    document.querySelector("#action-dialog-server .log_iframe").style.display = "none";
    let dialog = document.getElementById("action-dialog-server");
    dialog.headline = server;
    dialog.open = true;
}

// Log选项预设按钮
function Chip_Log_Options(id, logger_id) {
    let offset = document.querySelector(`#${logger_id} .LogOptionOffset`);
    let len = document.querySelector(`#${logger_id} .LogOptionLength`);
    switch (id) {
        case 1:
            offset.value = "0";
            len.value = "0";
            break;
        case 2:
            offset.value = "-1024";
            len.value = "0";
            break;
        case 3:
            offset.value = "-2048";
            len.value = "0";
            break;
        case 4:
            offset.value = "0";
            len.value = "1024";
            break;
    }
}

// 读取日志按钮
function readProcessLog(method, logger_id) {
    let offset = document.querySelector(`#${logger_id} .LogOptionOffset`).value;
    let len = document.querySelector(`#${logger_id} .LogOptionLength`).value;
    let url = "";
    if (method == "readLog") {
        url = `/log/${method}/${selected_server}?offset=${offset}&length=${len}&process=`
    } else {
        url = `/log/${method}/${dialog_server}?offset=${offset}&length=${len}&process=${dialog_program}`
    }
    if (document.querySelector(`#${logger_id} .OpenNewWindow`).checked) {
        window.open(url);
    } else {
        let lif = document.querySelector(`#${logger_id} .log_iframe`);
        lif.src = url;
        lif.style.display = "unset";
    }

}

// signalProcesses部分
function signalProcesses(target, logger_id, all) {
    let signal = document.querySelector(`#${logger_id} .ProcessesSignal`).value;
    let data = all ? { 'a': signal } : { 'a': dialog_program, 'b': signal };
    PostRequest(`/api/${dialog_server}/${all ? "signalAllProcess" : "signalProcess"}`, data, target, () => {
        CloseAllDialog();
        init();
    })
}

// 对象转表格函数
function ObjToTable(tag_id, obj) {
    let container = document.getElementById(tag_id);
    container.innerHTML = "";
    let table = document.createElement('table');
    for (let key in obj) {
        let row = document.createElement('tr');
        let keyCell = document.createElement('td');
        keyCell.textContent = key;
        row.appendChild(keyCell);
        let valueCell = document.createElement('td');
        valueCell.textContent = obj[key];
        row.appendChild(valueCell);
        table.appendChild(row);
    }
    container.appendChild(table);
}

function CommonWaitAction(cmd, target) {
    postMessage(`/api/${dialog_server}/${cmd}`, { 'a': true }, target, () => {
        CloseAllDialog();
        init();
    })
}

function CloseAllDialog() {
    document.getElementById("action-dialog-server").open = false;
    document.getElementById("action-dialog").open = false;
}

function reloadConfig(target) {
    PostRequest(`/api/${selected_server}/reloadConfig`, {}, target, res => {
        let tbody = document.getElementById("reloadConfigRusult");
        tbody.innerHTML = "";
        let len = Math.max(res["data"][0][0].length, res["data"][0][1].length, res["data"][0][1].length)
        for (let i = 0; i < len; i++) {
            let content = "<tr>";
            content += `<td>${res["data"][0][0][i] ? res["data"][0][0][i] : ""}</td>`;
            content += `<td>${res["data"][0][1][i] ? res["data"][0][1][i] : ""}</td>`;
            content += `<td>${res["data"][0][2][i] ? res["data"][0][2][i] : ""}</td>`;
            content += "</tr>";
            tbody.innerHTML += content;
        }
        document.getElementById("reloadConfigDialog").open = true;
    })
}

function clearLog(target, who) {
    let url = `/api/${who || who != "all" ? dialog_server : selected_server}/${who ? who == "all" ? "clearAllProcessLogs" : "clearProcessLogs" : "clearLog"}`
    let data = who || who != "all" ? { "a": dialog_program } : {};
    PostRequest(url, data, target);
}

function getConfig() {
    let node = document.getElementById("process-config").innerHTML = "<mdui-linear-progress></mdui-linear-progress>"
    PostRequest(`/api/${dialog_server}/getAllConfigInfo`, {}, undefined, res => {
        res["data"].forEach(item => {
            if (item["name"] == dialog_program) {
                ObjToTable("process-config", item);
            }
        })
    }, false)
}
document.querySelector('#info-tab mdui-collapse-item[value="item-2"]').addEventListener("open", getConfig);

function PostRequest(url, data, loading_node, success_callback, show_result = true) {
    if (loading_node) {
        loading_node.setAttribute("loading", "true");
    }
    mdui.$.ajax({
        url: url,
        method: 'post',
        data: JSON.stringify(data),
        contentType: 'application/json',
        dataType: 'json',
        success: function (res) {
            console.log(res);
            mdui.snackbar({ message: res["code"] ? `操作成功  ${show_result ? JSON.stringify(res["data"]) : ""}` : `操作失败：${JSON.stringify(res["data"])}`, placement: "top" });
            if (res["code"] && success_callback) {
                success_callback(res);
            }
            // CloseAllDialog();
            // init();
        },
        error: function (xhr, textStatus) {
            mdui.alert({ headline: "发送请求异常", description: textStatus });
        },
        complete: function () {
            if (loading_node) {
                loading_node.removeAttribute("loading");
            }
        }
    });
}

init();

// infos = {
//     "Processes": {
//         "127.0.0.1": null,
//         "192.168.1.100": [
//             {
//                 "description": "Dec 11 04:03 AM",
//                 "exitstatus": -1,
//                 "group": "chaoxing",
//                 "logfile": "/etc/supervisor/logs/stdout/chaoxing.log",
//                 "name": "chaoxing",
//                 "now": 1702626413,
//                 "pid": 0,
//                 "spawnerr": "",
//                 "start": 1702267405,
//                 "state": 0,
//                 "statename": "STOPPED",
//                 "stderr_logfile": "/etc/supervisor/logs/stderr/chaoxing.log",
//                 "stdout_logfile": "/etc/supervisor/logs/stdout/chaoxing.log",
//                 "stop": 1702267413
//             },
//             {
//                 "description": "pid 89, uptime 4 days, 3:43:10",
//                 "exitstatus": 0,
//                 "group": "ip_client",
//                 "logfile": "/etc/supervisor/logs/stdout/ip_client.log",
//                 "name": "ip_client",
//                 "now": 1702626413,
//                 "pid": 89,
//                 "spawnerr": "",
//                 "start": 1702267423,
//                 "state": 20,
//                 "statename": "RUNNING",
//                 "stderr_logfile": "/etc/supervisor/logs/stderr/ip_client.log",
//                 "stdout_logfile": "/etc/supervisor/logs/stdout/ip_client.log",
//                 "stop": 0
//             },
//             {
//                 "description": "Dec 14 12:47 PM",
//                 "exitstatus": -1,
//                 "group": "ping",
//                 "logfile": "/etc/supervisor/logs/stdout/ping.log",
//                 "name": "ping",
//                 "now": 1702626413,
//                 "pid": 0,
//                 "spawnerr": "",
//                 "start": 1702558037,
//                 "state": 0,
//                 "statename": "STOPPED",
//                 "stderr_logfile": "/etc/supervisor/logs/stderr/ping.log",
//                 "stdout_logfile": "/etc/supervisor/logs/stdout/ping.log",
//                 "stop": 1702558052
//             },
//             {
//                 "description": "pid 38, uptime 4 days, 3:44:02",
//                 "exitstatus": 0,
//                 "group": "proxy_server",
//                 "logfile": "/etc/supervisor/logs/stdout/proxy_server.log",
//                 "name": "proxy_server",
//                 "now": 1702626413,
//                 "pid": 38,
//                 "spawnerr": "",
//                 "start": 1702267371,
//                 "state": 20,
//                 "statename": "RUNNING",
//                 "stderr_logfile": "/etc/supervisor/logs/stderr/proxy_server.log",
//                 "stdout_logfile": "/etc/supervisor/logs/stdout/proxy_server.log",
//                 "stop": 0
//             }
//         ]
//     },
//     "Servers": {
//         "127.0.0.1": {
//             "api": null,
//             "identification": null,
//             "state": {
//                 "statecode": -2,
//                 "statename": "NetWorkError"
//             },
//             "version": null
//         },
//         "192.168.1.100": {
//             "api": "3.0",
//             "identification": "supervisor",
//             "state": {
//                 "statecode": 1,
//                 "statename": "RUNNING"
//             },
//             "version": "4.2.5"
//         }
//     }
// };
// init_generate();