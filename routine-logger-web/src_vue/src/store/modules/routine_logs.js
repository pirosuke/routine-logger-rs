import LogAPI from "../../api/routine_logs"

export default {
    namespaced: true,
    state: {
        logs: [],
        logCount: 0,
    },
    mutations: {
        setLogs(state, payload) {
            state.logs = payload.logs;
            state.logCount = payload.logCount;
        },
    },
    actions: {
        async fetchLogs(context, payload) {
            const logList = await LogAPI.fetchLogs(
              payload.page,
              payload.itemsPerPage,
            )

            let logCount = 0;
            if (logList.length > 0) {
                logCount = logList[0].log_count;
            }
            context.commit("setLogs", {
                logs: logList,
                logCount,
            });
        },
        async addLog(context, payload) {
            await LogAPI.addLog(payload.editedItem)
        },
        async deleteLog(context, payload) {
            await LogAPI.deleteLog(payload.editedItem.log_id)
        },
    },
}
