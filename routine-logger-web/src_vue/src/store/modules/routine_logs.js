import LogAPI from "../../api/routine_logs"

export default {
    namespaced: true,
    state: {
        logs: [],
    },
    mutations: {
        setLogs(state, payload) {
            state.logs = payload.logs;
        },
    },
    actions: {
        async fetchLogs(context) {
            const logList = await LogAPI.fetchLogs()
            context.commit("setLogs", {
                logs: logList,
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
