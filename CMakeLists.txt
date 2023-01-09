cmake_minimum_required(VERSION 3.16.0 FATAL_ERROR)

set(CMAKE_SYSTEM_VERSION 10.0 CACHE STRING "" FORCE)

project(projects CXX)

################################################################################
# Set target arch type if empty. Visual studio solution generator provides it.
################################################################################
if(NOT CMAKE_VS_PLATFORM_NAME)
    set(CMAKE_VS_PLATFORM_NAME "x64")
endif()
message("${CMAKE_VS_PLATFORM_NAME} architecture in use")

#if(NOT ("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "Win32"))
#    message(FATAL_ERROR "${CMAKE_VS_PLATFORM_NAME} arch is not supported!")
#endif()

################################################################################
# Global configuration types
################################################################################
set(CMAKE_CONFIGURATION_TYPES
        "Debug"
        "Release"
        CACHE STRING "" FORCE
        )

################################################################################
# Global compiler options
################################################################################
if(MSVC)
    # remove default flags provided with CMake for MSVC
    set(CMAKE_CXX_FLAGS "")
    set(CMAKE_CXX_FLAGS_DEBUG "")
    set(CMAKE_CXX_FLAGS_RELEASE "")
endif()

################################################################################
# Global linker options
################################################################################
if(MSVC)
    # remove default flags provided with CMake for MSVC
    set(CMAKE_EXE_LINKER_FLAGS "")
    set(CMAKE_MODULE_LINKER_FLAGS "")
    set(CMAKE_SHARED_LINKER_FLAGS "")
    set(CMAKE_STATIC_LINKER_FLAGS "")
    set(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS}")
    set(CMAKE_MODULE_LINKER_FLAGS_DEBUG "${CMAKE_MODULE_LINKER_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS_DEBUG "${CMAKE_SHARED_LINKER_FLAGS}")
    set(CMAKE_STATIC_LINKER_FLAGS_DEBUG "${CMAKE_STATIC_LINKER_FLAGS}")
    set(CMAKE_EXE_LINKER_FLAGS_RELEASE "${CMAKE_EXE_LINKER_FLAGS}")
    set(CMAKE_MODULE_LINKER_FLAGS_RELEASE "${CMAKE_MODULE_LINKER_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS_RELEASE "${CMAKE_SHARED_LINKER_FLAGS}")
    set(CMAKE_STATIC_LINKER_FLAGS_RELEASE "${CMAKE_STATIC_LINKER_FLAGS}")
endif()

################################################################################
# Nuget packages function stub.
################################################################################
function(use_package TARGET PACKAGE VERSION)
    message(WARNING "No implementation of use_package. Create yours. "
            "Package \"${PACKAGE}\" with version \"${VERSION}\" "
            "for target \"${TARGET}\" is ignored!")
endfunction()

################################################################################
# Common utils
################################################################################
include(CMake/Utils.cmake)

################################################################################
# Additional Global Settings(add specific info there)
################################################################################
include(CMake/GlobalSettingsInclude.cmake OPTIONAL)

################################################################################
# Use solution folders feature
################################################################################
set_property(GLOBAL PROPERTY USE_FOLDERS ON)

################################################################################
# Sub-projects
################################################################################



























set(PROJECT_NAME hldll)

################################################################################
# Source groups
################################################################################
set(Header_Files__common
        "common/beamdef.h"
        "common/com_model.h"
        "common/con_nprint.h"
        "common/const.h"
        "common/crc.h"
        "common/cvardef.h"
        "common/director_cmds.h"
        "common/dlight.h"
        "common/dll_state.h"
        "common/entity_state.h"
        "common/entity_types.h"
        "common/enums.h"
        "common/event_api.h"
        "common/event_args.h"
        "common/event_flags.h"
        "common/hltv.h"
        "common/in_buttons.h"
        "common/mathlib.h"
        "common/netadr.h"
        "common/particledef.h"
        "common/Platform.h"
        "common/PlatformHeaders.h"
        "common/r_efx.h"
        "common/r_studioint.h"
        "common/screenfade.h"
        "common/Sequence.h"
        "common/studio_event.h"
        "common/usercmd.h"
        "common/weaponinfo.h"
        )
source_group("Header Files\\common" FILES ${Header_Files__common})

set(Header_Files__dlls
        "dlls/activity.h"
        "dlls/activitymap.h"
        "dlls/animation.h"
        "dlls/basemonster.h"
        "dlls/cbase.h"
        "dlls/cdll_dll.h"
        "dlls/client.h"
        "dlls/decals.h"
        "dlls/defaultai.h"
        "dlls/doors.h"
        "dlls/effects.h"
        "dlls/enginecallback.h"
        "dlls/explode.h"
        "dlls/extdll.h"
        "dlls/flyingmonster.h"
        "dlls/func_break.h"
        "dlls/gamerules.h"
        "dlls/hornet.h"
        "dlls/items.h"
        "dlls/monsterevent.h"
        "dlls/monsters.h"
        "dlls/nodes.h"
        "dlls/plane.h"
        "dlls/player.h"
        "dlls/saverestore.h"
        "dlls/schedule.h"
        "dlls/scripted.h"
        "dlls/scriptevent.h"
        "dlls/skill.h"
        "dlls/soundent.h"
        "dlls/spectator.h"
        "dlls/squadmonster.h"
        "dlls/talkmonster.h"
        "dlls/teamplay_gamerules.h"
        "dlls/trains.h"
        "dlls/UserMessages.h"
        "dlls/util.h"
        "dlls/vector.h"
        "dlls/weapons.h"
        )
source_group("Header Files\\dlls" FILES ${Header_Files__dlls})

set(Header_Files__engine
        "engine/custom.h"
        "engine/customentity.h"
        "engine/edict.h"
        "engine/eiface.h"
        "engine/progdefs.h"
        "engine/progs.h"
        "engine/shake.h"
        "engine/studio.h"
        )
source_group("Header Files\\engine" FILES ${Header_Files__engine})

set(Header_Files__game_shared
        "game_shared/filesystem_utils.h"
        )
source_group("Header Files\\game_shared" FILES ${Header_Files__game_shared})

set(Header_Files__pm_shared
        "pm_shared/pm_debug.h"
        "pm_shared/pm_defs.h"
        "pm_shared/pm_info.h"
        "pm_shared/pm_materials.h"
        "pm_shared/pm_movevars.h"
        "pm_shared/pm_shared.h"
        )
source_group("Header Files\\pm_shared" FILES ${Header_Files__pm_shared})

set(Header_Files__public
        "public/interface.h"
        )
source_group("Header Files\\public" FILES ${Header_Files__public})

set(Source_Files__common
        "common/mathlib.cpp"
        )
source_group("Source Files\\common" FILES ${Source_Files__common})

set(Source_Files__dlls
        "dlls/aflock.cpp"
        "dlls/agrunt.cpp"
        "dlls/airtank.cpp"
        "dlls/animating.cpp"
        "dlls/animation.cpp"
        "dlls/apache.cpp"
        "dlls/barnacle.cpp"
        "dlls/barney.cpp"
        "dlls/bigmomma.cpp"
        "dlls/bloater.cpp"
        "dlls/bmodels.cpp"
        "dlls/bullsquid.cpp"
        "dlls/buttons.cpp"
        "dlls/cbase.cpp"
        "dlls/client.cpp"
        "dlls/combat.cpp"
        "dlls/controller.cpp"
        "dlls/defaultai.cpp"
        "dlls/doors.cpp"
        "dlls/effects.cpp"
        "dlls/egon.cpp"
        "dlls/explode.cpp"
        "dlls/flyingmonster.cpp"
        "dlls/func_break.cpp"
        "dlls/func_tank.cpp"
        "dlls/game.cpp"
        "dlls/gamerules.cpp"
        "dlls/gargantua.cpp"
        "dlls/genericmonster.cpp"
        "dlls/gman.cpp"
        "dlls/h_ai.cpp"
        "dlls/h_battery.cpp"
        "dlls/h_cine.cpp"
        "dlls/h_cycler.cpp"
        "dlls/h_export.cpp"
        "dlls/hassassin.cpp"
        "dlls/headcrab.cpp"
        "dlls/healthkit.cpp"
        "dlls/hgrunt.cpp"
        "dlls/hornet.cpp"
        "dlls/houndeye.cpp"
        "dlls/ichthyosaur.cpp"
        "dlls/islave.cpp"
        "dlls/items.cpp"
        "dlls/leech.cpp"
        "dlls/lights.cpp"
        "dlls/maprules.cpp"
        "dlls/monstermaker.cpp"
        "dlls/monsters.cpp"
        "dlls/monsterstate.cpp"
        "dlls/mortar.cpp"
        "dlls/mp5.cpp"
        "dlls/multiplay_gamerules.cpp"
        "dlls/nihilanth.cpp"
        "dlls/nodes.cpp"
        "dlls/observer.cpp"
        "dlls/osprey.cpp"
        "dlls/pathcorner.cpp"
        "dlls/plane.cpp"
        "dlls/plats.cpp"
        "dlls/player.cpp"
        "dlls/python.cpp"
        "dlls/rat.cpp"
        "dlls/roach.cpp"
        "dlls/satchel.cpp"
        "dlls/schedule.cpp"
        "dlls/scientist.cpp"
        "dlls/scripted.cpp"
        "dlls/singleplay_gamerules.cpp"
        "dlls/skill.cpp"
        "dlls/sound.cpp"
        "dlls/soundent.cpp"
        "dlls/spectator.cpp"
        "dlls/squadmonster.cpp"
        "dlls/squeakgrenade.cpp"
        "dlls/subs.cpp"
        "dlls/talkmonster.cpp"
        "dlls/teamplay_gamerules.cpp"
        "dlls/tempmonster.cpp"
        "dlls/tentacle.cpp"
        "dlls/triggers.cpp"
        "dlls/tripmine.cpp"
        "dlls/turret.cpp"
        "dlls/UserMessages.cpp"
        "dlls/util.cpp"
        "dlls/world.cpp"
        "dlls/xen.cpp"
        "dlls/zombie.cpp"
        )
source_group("Source Files\\dlls" FILES ${Source_Files__dlls})

set(Source_Files__dlls__weapons
        "dlls/crossbow.cpp"
        "dlls/crowbar.cpp"
        "dlls/gauss.cpp"
        "dlls/ggrenade.cpp"
        "dlls/glock.cpp"
        "dlls/handgrenade.cpp"
        "dlls/hornetgun.cpp"
        "dlls/rpg.cpp"
        "dlls/shotgun.cpp"
        "dlls/weapons.cpp"
        "dlls/weapons_shared.cpp"
        )
source_group("Source Files\\dlls\\weapons" FILES ${Source_Files__dlls__weapons})

set(Source_Files__game_shared
        "game_shared/filesystem_utils.cpp"
        "game_shared/voice_gamemgr.cpp"
        )
source_group("Source Files\\game_shared" FILES ${Source_Files__game_shared})

set(Source_Files__pm_shared
        "pm_shared/pm_debug.cpp"
        "pm_shared/pm_math.cpp"
        "pm_shared/pm_shared.cpp"
        )
source_group("Source Files\\pm_shared" FILES ${Source_Files__pm_shared})

set(Source_Files__public
        "public/interface.cpp"
        )
source_group("Source Files\\public" FILES ${Source_Files__public})

set(ALL_FILES
        ${Header_Files__common}
        ${Header_Files__dlls}
        ${Header_Files__engine}
        ${Header_Files__game_shared}
        ${Header_Files__pm_shared}
        ${Header_Files__public}
        ${Source_Files__common}
        ${Source_Files__dlls}
        ${Source_Files__dlls__weapons}
        ${Source_Files__game_shared}
        ${Source_Files__pm_shared}
        ${Source_Files__public}
        )

################################################################################
# Target
################################################################################
add_library(${PROJECT_NAME} SHARED ${ALL_FILES})

use_props(${PROJECT_NAME} "${CMAKE_CONFIGURATION_TYPES}" "${DEFAULT_CXX_PROPS}")
set(ROOT_NAMESPACE hldll)

set_target_properties(${PROJECT_NAME} PROPERTIES
        VS_GLOBAL_KEYWORD "Win32Proj"
        )
################################################################################
# Target name
################################################################################
set_target_properties(${PROJECT_NAME} PROPERTIES
        TARGET_NAME_DEBUG   "hl"
        TARGET_NAME_RELEASE "hl"
        )
################################################################################
# Output directory
################################################################################
set_target_properties(${PROJECT_NAME} PROPERTIES
        OUTPUT_DIRECTORY_DEBUG   "${CMAKE_SOURCE_DIR}/$<CONFIG>/${PROJECT_NAME}/"
        OUTPUT_DIRECTORY_RELEASE "${CMAKE_SOURCE_DIR}/$<CONFIG>/${PROJECT_NAME}/"
        )
set_target_properties(${PROJECT_NAME} PROPERTIES
        INTERPROCEDURAL_OPTIMIZATION_RELEASE "TRUE"
        )
################################################################################
# MSVC runtime library
################################################################################
get_property(MSVC_RUNTIME_LIBRARY_DEFAULT TARGET ${PROJECT_NAME} PROPERTY MSVC_RUNTIME_LIBRARY)
string(CONCAT "MSVC_RUNTIME_LIBRARY_STR"
        $<$<CONFIG:Debug>:
        MultiThreadedDebug
        >
        $<$<CONFIG:Release>:
        MultiThreaded
        >
        $<$<NOT:$<OR:$<CONFIG:Debug>,$<CONFIG:Release>>>:${MSVC_RUNTIME_LIBRARY_DEFAULT}>
        )
set_target_properties(${PROJECT_NAME} PROPERTIES MSVC_RUNTIME_LIBRARY ${MSVC_RUNTIME_LIBRARY_STR})

################################################################################
# Include directories
################################################################################
target_include_directories(${PROJECT_NAME} PUBLIC
        "${CMAKE_CURRENT_SOURCE_DIR}/dlls;"
        "${CMAKE_CURRENT_SOURCE_DIR}/engine;"
        "${CMAKE_CURRENT_SOURCE_DIR}/common;"
        "${CMAKE_CURRENT_SOURCE_DIR}/pm_shared;"
        "${CMAKE_CURRENT_SOURCE_DIR}/game_shared;"
        "${CMAKE_CURRENT_SOURCE_DIR}/public"
        )

################################################################################
# Compile definitions
################################################################################
target_compile_definitions(${PROJECT_NAME} PRIVATE
        "$<$<CONFIG:Debug>:"
        "_DEBUG"
        ">"
        "$<$<CONFIG:Release>:"
        "NDEBUG"
        ">"
        "WIN32;"
        "_CRT_SECURE_NO_WARNINGS;"
        "_WINDOWS;"
        "QUIVER;"
        "VOXEL;"
        "QUAKE2;"
        "VALVE_DLL;"
        "CLIENT_WEAPONS"
        )

################################################################################
# Compile and link options
################################################################################
if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE
            $<$<CONFIG:Debug>:
            /Od
            >
            $<$<CONFIG:Release>:
            /O2;
            /Oi;
            /Gy
            >
            /MP;
            /std:c++17;
            /W3;
            ${DEFAULT_CXX_DEBUG_INFORMATION_FORMAT};
            /GR;
            /Zc:threadSafeInit-;
            ${DEFAULT_CXX_EXCEPTION_HANDLING};
            /Y-
            )
    target_link_options(${PROJECT_NAME} PRIVATE
            $<$<CONFIG:Debug>:
            /INCREMENTAL
            >
            $<$<CONFIG:Release>:
            /OPT:REF;
            /OPT:ICF;
            /INCREMENTAL:NO
            >
            /DEBUG;
            /SUBSYSTEM:WINDOWS
            )
endif()


























set(PROJECT_NAME hl_cdll)

################################################################################
# Source groups
################################################################################
set(Header_Files__cl_dll
        "cl_dll/ammo.h"
        "cl_dll/ammohistory.h"
        "cl_dll/camera.h"
        "cl_dll/cl_dll.h"
        "cl_dll/cl_util.h"
        "cl_dll/com_weapons.h"
        "cl_dll/demo.h"
        "cl_dll/ev_hldm.h"
        "cl_dll/eventscripts.h"
        "cl_dll/GameStudioModelRenderer.h"
        "cl_dll/health.h"
        "cl_dll/hud.h"
        "cl_dll/hud_spectator.h"
        "cl_dll/in_defs.h"
        "cl_dll/interpolation.h"
        "cl_dll/kbutton.h"
        "cl_dll/StudioModelRenderer.h"
        "cl_dll/tri.h"
        "cl_dll/vgui_int.h"
        "cl_dll/vgui_SchemeManager.h"
        "cl_dll/vgui_ScorePanel.h"
        "cl_dll/vgui_SpectatorPanel.h"
        "cl_dll/view.h"
        "dlls/cdll_dll.h"
        )
source_group("Header Files\\cl_dll" FILES ${Header_Files__cl_dll})

set(Header_Files__cl_dll__particleman
        "cl_dll/particleman/CBaseParticle.h"
        "cl_dll/particleman/CFrustum.h"
        "cl_dll/particleman/CMiniMem.h"
        "cl_dll/particleman/IParticleMan_Active.h"
        "cl_dll/particleman/particleman.h"
        "cl_dll/particleman/particleman_internal.h"
        )
source_group("Header Files\\cl_dll\\particleman" FILES ${Header_Files__cl_dll__particleman})

set(Header_Files__common
        "common/beamdef.h"
        "common/cl_entity.h"
        "common/com_model.h"
        "common/common_types.h"
        "common/con_nprint.h"
        "common/const.h"
        "common/crc.h"
        "common/cvardef.h"
        "common/demo_api.h"
        "common/director_cmds.h"
        "common/dlight.h"
        "common/dll_state.h"
        "common/entity_state.h"
        "common/entity_types.h"
        "common/enums.h"
        "common/event_api.h"
        "common/event_args.h"
        "common/event_flags.h"
        "common/hltv.h"
        "common/in_buttons.h"
        "common/ivoicetweak.h"
        "common/mathlib.h"
        "common/net_api.h"
        "common/netadr.h"
        "common/parsemsg.h"
        "common/particledef.h"
        "common/Platform.h"
        "common/PlatformHeaders.h"
        "common/pmtrace.h"
        "common/qfont.h"
        "common/r_efx.h"
        "common/r_studioint.h"
        "common/ref_params.h"
        "common/screenfade.h"
        "common/Sequence.h"
        "common/studio_event.h"
        "common/triangleapi.h"
        "common/usercmd.h"
        "common/weaponinfo.h"
        )
source_group("Header Files\\common" FILES ${Header_Files__common})

set(Header_Files__engine
        "engine/APIProxy.h"
        "engine/cdll_int.h"
        "engine/custom.h"
        "engine/customentity.h"
        "engine/edict.h"
        "engine/eiface.h"
        "engine/progdefs.h"
        "engine/progs.h"
        "engine/shake.h"
        "engine/studio.h"
        )
source_group("Header Files\\engine" FILES ${Header_Files__engine})

set(Header_Files__game_shared
        "game_shared/filesystem_utils.h"
        "game_shared/vgui_scrollbar2.h"
        "game_shared/vgui_slider2.h"
        "game_shared/voice_banmgr.h"
        "game_shared/voice_status.h"
        )
source_group("Header Files\\game_shared" FILES ${Header_Files__game_shared})

set(Header_Files__pm_shared
        "pm_shared/pm_debug.h"
        "pm_shared/pm_defs.h"
        "pm_shared/pm_info.h"
        "pm_shared/pm_materials.h"
        "pm_shared/pm_movevars.h"
        "pm_shared/pm_shared.h"
        )
source_group("Header Files\\pm_shared" FILES ${Header_Files__pm_shared})

set(Header_Files__public
        "public/interface.h"
        )
source_group("Header Files\\public" FILES ${Header_Files__public})

set(Source_Files___hl__cl_dll
        "cl_dll/ev_hldm.cpp"
        )
source_group("Source Files\\_hl\\cl_dll" FILES ${Source_Files___hl__cl_dll})

set(Source_Files___hl__cl_dll__hl
        "cl_dll/hl/hl_baseentity.cpp"
        "cl_dll/hl/hl_events.cpp"
        "cl_dll/hl/hl_objects.cpp"
        "cl_dll/hl/hl_weapons.cpp"
        )
source_group("Source Files\\_hl\\cl_dll\\hl" FILES ${Source_Files___hl__cl_dll__hl})

set(Source_Files___hl__dlls
        "dlls/crossbow.cpp"
        "dlls/crowbar.cpp"
        "dlls/egon.cpp"
        "dlls/gauss.cpp"
        "dlls/glock.cpp"
        "dlls/handgrenade.cpp"
        "dlls/hornetgun.cpp"
        "dlls/mp5.cpp"
        "dlls/python.cpp"
        "dlls/rpg.cpp"
        "dlls/satchel.cpp"
        "dlls/shotgun.cpp"
        "dlls/squeakgrenade.cpp"
        "dlls/tripmine.cpp"
        "dlls/weapons_shared.cpp"
        )
source_group("Source Files\\_hl\\dlls" FILES ${Source_Files___hl__dlls})

set(Source_Files__cl_dll
        "cl_dll/ammo.cpp"
        "cl_dll/ammo_secondary.cpp"
        "cl_dll/ammohistory.cpp"
        "cl_dll/battery.cpp"
        "cl_dll/cdll_int.cpp"
        "cl_dll/com_weapons.cpp"
        "cl_dll/death.cpp"
        "cl_dll/demo.cpp"
        "cl_dll/entity.cpp"
        "cl_dll/ev_common.cpp"
        "cl_dll/events.cpp"
        "cl_dll/flashlight.cpp"
        "cl_dll/GameStudioModelRenderer.cpp"
        "cl_dll/geiger.cpp"
        "cl_dll/health.cpp"
        "cl_dll/hud.cpp"
        "cl_dll/hud_msg.cpp"
        "cl_dll/hud_redraw.cpp"
        "cl_dll/hud_spectator.cpp"
        "cl_dll/hud_update.cpp"
        "cl_dll/in_camera.cpp"
        "cl_dll/input.cpp"
        "cl_dll/inputw32.cpp"
        "cl_dll/interpolation.cpp"
        "cl_dll/menu.cpp"
        "cl_dll/message.cpp"
        "cl_dll/saytext.cpp"
        "cl_dll/status_icons.cpp"
        "cl_dll/statusbar.cpp"
        "cl_dll/studio_util.cpp"
        "cl_dll/StudioModelRenderer.cpp"
        "cl_dll/text_message.cpp"
        "cl_dll/train.cpp"
        "cl_dll/tri.cpp"
        "cl_dll/util.cpp"
        "cl_dll/vgui_ClassMenu.cpp"
        "cl_dll/vgui_CustomObjects.cpp"
        "cl_dll/vgui_int.cpp"
        "cl_dll/vgui_MOTDWindow.cpp"
        "cl_dll/vgui_SchemeManager.cpp"
        "cl_dll/vgui_ScorePanel.cpp"
        "cl_dll/vgui_SpectatorPanel.cpp"
        "cl_dll/vgui_TeamFortressViewport.cpp"
        "cl_dll/vgui_teammenu.cpp"
        "cl_dll/view.cpp"
        "cl_dll/voice_status.cpp"
        )
source_group("Source Files\\cl_dll" FILES ${Source_Files__cl_dll})

set(Source_Files__cl_dll__particleman
        "cl_dll/particleman/CBaseParticle.cpp"
        "cl_dll/particleman/CFrustum.cpp"
        "cl_dll/particleman/CMiniMem.cpp"
        "cl_dll/particleman/IParticleMan_Active.cpp"
        )
source_group("Source Files\\cl_dll\\particleman" FILES ${Source_Files__cl_dll__particleman})

set(Source_Files__common
        "common/mathlib.cpp"
        "common/parsemsg.cpp"
        )
source_group("Source Files\\common" FILES ${Source_Files__common})

set(Source_Files__game_shared
        "game_shared/filesystem_utils.cpp"
        "game_shared/vgui_checkbutton2.cpp"
        "game_shared/vgui_grid.cpp"
        "game_shared/vgui_helpers.cpp"
        "game_shared/vgui_listbox.cpp"
        "game_shared/vgui_loadtga.cpp"
        "game_shared/vgui_scrollbar2.cpp"
        "game_shared/vgui_slider2.cpp"
        "game_shared/voice_banmgr.cpp"
        )
source_group("Source Files\\game_shared" FILES ${Source_Files__game_shared})

set(Source_Files__pm_shared
        "pm_shared/pm_debug.cpp"
        "pm_shared/pm_math.cpp"
        "pm_shared/pm_shared.cpp"
        )
source_group("Source Files\\pm_shared" FILES ${Source_Files__pm_shared})

set(Source_Files__public
        "public/interface.cpp"
        )
source_group("Source Files\\public" FILES ${Source_Files__public})

set(ALL_FILES
        ${Header_Files__cl_dll}
        ${Header_Files__cl_dll__particleman}
        ${Header_Files__common}
        ${Header_Files__engine}
        ${Header_Files__game_shared}
        ${Header_Files__pm_shared}
        ${Header_Files__public}
        ${Source_Files___hl__cl_dll}
        ${Source_Files___hl__cl_dll__hl}
        ${Source_Files___hl__dlls}
        ${Source_Files__cl_dll}
        ${Source_Files__cl_dll__particleman}
        ${Source_Files__common}
        ${Source_Files__game_shared}
        ${Source_Files__pm_shared}
        ${Source_Files__public}
        )

################################################################################
# Target
################################################################################
add_library(${PROJECT_NAME} SHARED ${ALL_FILES})

use_props(${PROJECT_NAME} "${CMAKE_CONFIGURATION_TYPES}" "${DEFAULT_CXX_PROPS}")
set(ROOT_NAMESPACE hl_cdll)

set_target_properties(${PROJECT_NAME} PROPERTIES
        VS_GLOBAL_KEYWORD "Win32Proj"
        )
################################################################################
# Target name
################################################################################
set_target_properties(${PROJECT_NAME} PROPERTIES
        TARGET_NAME_DEBUG   "client"
        TARGET_NAME_RELEASE "client"
        )
################################################################################
# Output directory
################################################################################
set_target_properties(${PROJECT_NAME} PROPERTIES
        OUTPUT_DIRECTORY_DEBUG   "${CMAKE_SOURCE_DIR}/$<CONFIG>//${PROJECT_NAME}/"
        OUTPUT_DIRECTORY_RELEASE "${CMAKE_SOURCE_DIR}/$<CONFIG>//${PROJECT_NAME}/"
        )
set_target_properties(${PROJECT_NAME} PROPERTIES
        INTERPROCEDURAL_OPTIMIZATION_RELEASE "TRUE"
        )
################################################################################
# MSVC runtime library
################################################################################
get_property(MSVC_RUNTIME_LIBRARY_DEFAULT TARGET ${PROJECT_NAME} PROPERTY MSVC_RUNTIME_LIBRARY)
string(CONCAT "MSVC_RUNTIME_LIBRARY_STR"
        $<$<CONFIG:Debug>:
        MultiThreadedDebug
        >
        $<$<CONFIG:Release>:
        MultiThreaded
        >
        $<$<NOT:$<OR:$<CONFIG:Debug>,$<CONFIG:Release>>>:${MSVC_RUNTIME_LIBRARY_DEFAULT}>
        )
set_target_properties(${PROJECT_NAME} PROPERTIES MSVC_RUNTIME_LIBRARY ${MSVC_RUNTIME_LIBRARY_STR})

################################################################################
# Include directories
################################################################################
target_include_directories(${PROJECT_NAME} PUBLIC
        "${CMAKE_CURRENT_SOURCE_DIR}/dlls;"
        "${CMAKE_CURRENT_SOURCE_DIR}/cl_dll;"
        "${CMAKE_CURRENT_SOURCE_DIR}/cl_dll/particleman;"
        "${CMAKE_CURRENT_SOURCE_DIR}/public;"
        "${CMAKE_CURRENT_SOURCE_DIR}/common;"
        "${CMAKE_CURRENT_SOURCE_DIR}/pm_shared;"
        "${CMAKE_CURRENT_SOURCE_DIR}/engine;"
        "${CMAKE_CURRENT_SOURCE_DIR}/utils/vgui/include;"
        "${CMAKE_CURRENT_SOURCE_DIR}/game_shared;"
        "${CMAKE_CURRENT_SOURCE_DIR}/external"
        )

################################################################################
# Compile definitions
################################################################################
target_compile_definitions(${PROJECT_NAME} PRIVATE
        "$<$<CONFIG:Debug>:"
        "_DEBUG"
        ">"
        "$<$<CONFIG:Release>:"
        "NDEBUG"
        ">"
        "WIN32;"
        "_CRT_SECURE_NO_WARNINGS;"
        "_WINDOWS;"
        "CLIENT_DLL;"
        "CLIENT_WEAPONS;"
        "HL_DLL"
        )

################################################################################
# Compile and link options
################################################################################
if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE
            $<$<CONFIG:Debug>:
            /Od
            >
            $<$<CONFIG:Release>:
            /O2;
            /Oi;
            /Gy
            >
            /MP;
            /std:c++17;
            /W3;
            ${DEFAULT_CXX_DEBUG_INFORMATION_FORMAT};
            /GR;
            /Zc:threadSafeInit-;
            ${DEFAULT_CXX_EXCEPTION_HANDLING};
            /Y-
            )
    target_link_options(${PROJECT_NAME} PRIVATE
            $<$<CONFIG:Debug>:
            /INCREMENTAL
            >
            $<$<CONFIG:Release>:
            /OPT:REF;
            /OPT:ICF;
            /INCREMENTAL:NO
            >
            /DEBUG;
            /SUBSYSTEM:WINDOWS
            )
endif()


################################################################################
# Dependencies
################################################################################
add_dependencies(${PROJECT_NAME} hldll)

target_link_libraries(${PROJECT_NAME}
        PUBLIC utils/vgui/lib/win32_vc6/vgui
        PUBLIC wsock32
        PUBLIC lib/public/sdl2
 )

