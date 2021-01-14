import axios from "axios"

const apiServer = axios.create({
    baseURL: process.env.VUE_APP_AEC_BASE_URL,
})

export default {
    async fetchLogs() {
        const res = await apiServer.get("/routine_logs", {
            responseType: "json",
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
