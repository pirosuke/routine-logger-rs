import axios from "axios"

const apiServer = axios.create({
    baseURL: process.env.VUE_APP_AEC_BASE_URL,
})

export default {
    async fetchRoutines() {
        const res = await apiServer.get("/routines", {
            responseType: "json",
        })
        return res.data
    },

    async addRoutine(item) {
        const res = await apiServer.post(
            "/routines",
            item,
        )
    },
    async updateRoutine(item) {
        const res = await apiServer.put(
            "/routines",
            item,
        )
    },
    async deleteRoutine(routineId) {
        const res = await apiServer.delete(
            `/routines/${routineId}`,
        )
    },
}