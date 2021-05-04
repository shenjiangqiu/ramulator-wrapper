#include<queue>
#include "DRAMSim.h"

extern "C"
{
   

    int dramsim2_get_channel_id(void *,unsigned long long addr);
    void *get_dramsim2();
    bool dramsim2_send(void *dramsim2, unsigned long long addr, bool is_write);
    unsigned long long dramsim2_get(void *dramsim2);
    void dramsim2_tick(void *dramsim2);
    bool dramsim2_ret_available(void *);
    bool dramsim2_available(void *,unsigned long long addr, bool is_write);
    void delete_dramsim2(void *);
}
