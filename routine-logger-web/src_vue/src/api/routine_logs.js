import axios from "axios"

const apiServer = axios.create({
    baseURL: process.env.VUE_APP_AEC_BASE_URL,
})

export default {
    async fetchLogs(page, itemsPerPage) {
        const res = await apiServer.get("/routine_logs", {
            responseType: "json",
            params: {
              page,
              limit: itemsPerPage,
            }
        })
        return res.data
    },

    async addLog(item) {
        const res = await apiServer.post(
            "/routine_logs",
            item,
        )
    },
    async deleteLog(logId) {
        const res = await apiServer.delete(
            `/routine_logs/${logId}`,
        )
    },
}
